# AST Trace: ../rust/compiler/rustc_session/src/search_paths.rs

Generated 8 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::path::{Path, PathBuf};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use std::sync::Arc;

use rustc_macros::{Decodable, Encodable, HashStable_Generic};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=SearchPath | COMPLEXITY=2 | LINES=11

```rust
use rustc_target::spec::TargetTuple;

use crate::EarlyDiagCtxt;
use crate::filesearch::make_target_lib_path;

#[derive(Clone, Debug)]
pub struct SearchPath {
    pub kind: PathKind,
    pub dir: PathBuf,
    pub files: FilesIndex,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=FilesIndex(Vec | COMPLEXITY=11 | LINES=35

```rust
/// [FilesIndex] contains paths that can be efficiently looked up with (prefix, suffix) pairs.
#[derive(Clone, Debug)]
pub struct FilesIndex(Vec<(Arc<str>, SearchPathFile)>);

impl FilesIndex {
    /// Look up [SearchPathFile] by (prefix, suffix) pair.
    pub fn query<'s>(
        &'s self,
        prefix: &str,
        suffix: &str,
    ) -> Option<impl Iterator<Item = (String, &'s SearchPathFile)>> {
        let start = self.0.partition_point(|(k, _)| **k < *prefix);
        if start == self.0.len() {
            return None;
        }
        let end = self.0[start..].partition_point(|(k, _)| k.starts_with(prefix));
        let prefixed_items = &self.0[start..][..end];

        let ret = prefixed_items.into_iter().filter_map(move |(k, v)| {
            k.ends_with(suffix).then(|| {
                (
                    String::from(
                        &v.file_name_str[prefix.len()..v.file_name_str.len() - suffix.len()],
                    ),
                    v,
                )
            })
        });
        Some(ret)
    }
    pub fn retain(&mut self, prefixes: &[&str]) {
        self.0.retain(|(k, _)| prefixes.iter().any(|prefix| k.starts_with(prefix)));
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=SearchPathFile | COMPLEXITY=3 | LINES=16

```rust
/// The obvious implementation of `SearchPath::files` is a `Vec<PathBuf>`. But
/// it is searched repeatedly by `find_library_crate`, and the searches involve
/// checking the prefix and suffix of the filename of each `PathBuf`. This is
/// doable, but very slow, because it involves calls to `file_name` and
/// `extension` that are themselves slow.
///
/// This type augments the `PathBuf` with an `String` containing the
/// `PathBuf`'s filename. The prefix and suffix checking is much faster on the
/// `String` than the `PathBuf`. (The filename must be valid UTF-8. If it's
/// not, the entry should be skipped, because all Rust output files are valid
/// UTF-8, and so a non-UTF-8 filename couldn't be one we're looking for.)
#[derive(Clone, Debug)]
pub struct SearchPathFile {
    pub path: Arc<Path>,
    pub file_name_str: Arc<str>,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq, Encodable, Decodable, HashStable_Generic)]
pub enum PathKind {
    Native,
    Crate,
    Dependency,
    Framework,
    ExternFlag,
    All,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=matches | COMPLEXITY=7 | LINES=9

```rust
impl PathKind {
    pub fn matches(&self, kind: PathKind) -> bool {
        match (self, kind) {
            (PathKind::All, _) | (_, PathKind::All) => true,
            _ => *self == kind,
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=from_cli_opt | COMPLEXITY=46 | LINES=72

```rust
impl SearchPath {
    pub fn from_cli_opt(
        sysroot: &Path,
        triple: &TargetTuple,
        early_dcx: &EarlyDiagCtxt,
        path: &str,
        is_unstable_enabled: bool,
    ) -> Self {
        let (kind, path) = if let Some(stripped) = path.strip_prefix("native=") {
            (PathKind::Native, stripped)
        } else if let Some(stripped) = path.strip_prefix("crate=") {
            (PathKind::Crate, stripped)
        } else if let Some(stripped) = path.strip_prefix("dependency=") {
            (PathKind::Dependency, stripped)
        } else if let Some(stripped) = path.strip_prefix("framework=") {
            (PathKind::Framework, stripped)
        } else if let Some(stripped) = path.strip_prefix("all=") {
            (PathKind::All, stripped)
        } else {
            (PathKind::All, path)
        };
        let dir = match path.strip_prefix("@RUSTC_BUILTIN") {
            Some(stripped) => {
                if !is_unstable_enabled {
                    #[allow(rustc::untranslatable_diagnostic)] // FIXME: make this translatable
                    early_dcx.early_fatal(
                        "the `-Z unstable-options` flag must also be passed to \
                         enable the use of `@RUSTC_BUILTIN`",
                    );
                }

                make_target_lib_path(sysroot, triple.tuple()).join("builtin").join(stripped)
            }
            None => PathBuf::from(path),
        };
        if dir.as_os_str().is_empty() {
            #[allow(rustc::untranslatable_diagnostic)] // FIXME: make this translatable
            early_dcx.early_fatal("empty search path given via `-L`");
        }

        Self::new(kind, dir)
    }

    pub fn from_sysroot_and_triple(sysroot: &Path, triple: &str) -> Self {
        Self::new(PathKind::All, make_target_lib_path(sysroot, triple))
    }

    pub fn new(kind: PathKind, dir: PathBuf) -> Self {
        // Get the files within the directory.
        let mut files = match std::fs::read_dir(&dir) {
            Ok(files) => files
                .filter_map(|e| {
                    e.ok().and_then(|e| {
                        e.file_name().to_str().map(|s| {
                            let file_name_str: Arc<str> = s.into();
                            (
                                Arc::clone(&file_name_str),
                                SearchPathFile { path: e.path().into(), file_name_str },
                            )
                        })
                    })
                })
                .collect::<Vec<_>>(),

            Err(..) => Default::default(),
        };
        files.sort_by(|(lhs, _), (rhs, _)| lhs.cmp(rhs));
        let files = FilesIndex(files);
        SearchPath { kind, dir, files }
    }
}
```

---
*Generated by AST tracing system*
