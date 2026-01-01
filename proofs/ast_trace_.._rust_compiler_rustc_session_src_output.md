# AST Trace: ../rust/compiler/rustc_session/src/output.rs

Generated 13 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
// Related to out filenames of compilation (e.g. binaries).

use std::path::Path;

use rustc_ast as ast;
use crate::rustc_complete::{Span, Symbol, sym};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use crate::Session;
use crate::config::{self, CrateType, OutFileName, OutputFilenames, OutputType};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::errors::{self, CrateNameEmpty, FileIsNotWriteable, InvalidCharacterInCrateName};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=out_filename | COMPLEXITY=6 | LINES=21

```rust
pub fn out_filename(
    sess: &Session,
    crate_type: CrateType,
    outputs: &OutputFilenames,
    crate_name: Symbol,
) -> OutFileName {
    let default_filename = filename_for_input(sess, crate_type, crate_name, outputs);
    let out_filename = outputs
        .outputs
        .get(&OutputType::Exe)
        .and_then(|s| s.to_owned())
        .or_else(|| outputs.single_output_file.clone())
        .unwrap_or(default_filename);

    if let OutFileName::Real(ref path) = out_filename {
        check_file_is_writeable(path, sess);
    }

    out_filename
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=check_file_is_writeable | COMPLEXITY=6 | LINES=9

```rust
/// Make sure files are writeable. Mac, FreeBSD, and Windows system linkers
/// check this already -- however, the Linux linker will happily overwrite a
/// read-only file. We should be consistent.
pub fn check_file_is_writeable(file: &Path, sess: &Session) {
    if !is_writeable(file) {
        sess.dcx().emit_fatal(FileIsNotWriteable { file });
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=is_writeable | COMPLEXITY=6 | LINES=7

```rust
fn is_writeable(p: &Path) -> bool {
    match p.metadata() {
        Err(..) => true,
        Ok(m) => !m.permissions().readonly(),
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=validate_crate_name | COMPLEXITY=18 | LINES=32

```rust
/// Validate the given crate name.
///
/// Note that this validation is more permissive than identifier parsing. It considers
/// non-empty sequences of alphanumeric and underscore characters to be valid crate names.
/// Most notably, it accepts names starting with a numeric character like `0`!
///
/// Furthermore, this shouldn't be taken as the canonical crate name validator.
/// Other places may use a more restrictive grammar (e.g., identifier or ASCII identifier).
pub fn validate_crate_name(sess: &Session, crate_name: Symbol, span: Option<Span>) {
    let mut guar = None;

    if crate_name.is_empty() {
        guar = Some(sess.dcx().emit_err(CrateNameEmpty { span }));
    }

    for c in crate_name.as_str().chars() {
        if c.is_alphanumeric() || c == '_' {
            continue;
        }
        guar = Some(sess.dcx().emit_err(InvalidCharacterInCrateName {
            span,
            character: c,
            crate_name,
            help: span.is_none().then_some(()),
        }));
    }

    if let Some(guar) = guar {
        guar.raise_fatal();
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=filename_for_metadata | COMPLEXITY=5 | LINES=8

```rust
pub fn filename_for_metadata(sess: &Session, outputs: &OutputFilenames) -> OutFileName {
    let out_filename = outputs.path(OutputType::Metadata);
    if let OutFileName::Real(ref path) = out_filename {
        check_file_is_writeable(path, sess);
    }
    out_filename
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=filename_for_input | COMPLEXITY=29 | LINES=36

```rust
pub fn filename_for_input(
    sess: &Session,
    crate_type: CrateType,
    crate_name: Symbol,
    outputs: &OutputFilenames,
) -> OutFileName {
    let libname = format!("{}{}", crate_name, sess.opts.cg.extra_filename);

    match crate_type {
        CrateType::Rlib => {
            OutFileName::Real(outputs.out_directory.join(&format!("lib{libname}.rlib")))
        }
        CrateType::Cdylib | CrateType::ProcMacro | CrateType::Dylib | CrateType::Sdylib => {
            let (prefix, suffix) = (&sess.target.dll_prefix, &sess.target.dll_suffix);
            OutFileName::Real(outputs.out_directory.join(&format!("{prefix}{libname}{suffix}")))
        }
        CrateType::Staticlib => {
            let (prefix, suffix) = sess.staticlib_components(false);
            OutFileName::Real(outputs.out_directory.join(&format!("{prefix}{libname}{suffix}")))
        }
        CrateType::Executable => {
            let suffix = &sess.target.exe_suffix;
            let out_filename = outputs.path(OutputType::Exe);
            if let OutFileName::Real(ref path) = out_filename {
                if suffix.is_empty() {
                    out_filename
                } else {
                    OutFileName::Real(path.with_extension(&suffix[1..]))
                }
            } else {
                out_filename
            }
        }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=default_output_for_target | COMPLEXITY=13 | LINES=13

```rust
/// Returns default crate type for target
///
/// Default crate type is used when crate type isn't provided neither
/// through cmd line arguments nor through crate attributes
///
/// It is CrateType::Executable for all platforms but iOS as there is no
/// way to run iOS binaries anyway without jailbreaking and
/// interaction with Rust code through static library is the only
/// option for now
pub fn default_output_for_target(sess: &Session) -> CrateType {
    if !sess.target.executables { CrateType::Staticlib } else { CrateType::Executable }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=invalid_output_for_target | COMPLEXITY=20 | LINES=24

```rust
/// Checks if target supports crate_type as output
pub fn invalid_output_for_target(sess: &Session, crate_type: CrateType) -> bool {
    if let CrateType::Cdylib | CrateType::Dylib | CrateType::ProcMacro = crate_type {
        if !sess.target.dynamic_linking {
            return true;
        }
        if sess.crt_static(Some(crate_type)) && !sess.target.crt_static_allows_dylibs {
            return true;
        }
    }
    if let CrateType::ProcMacro | CrateType::Dylib = crate_type
        && sess.target.only_cdylib
    {
        return true;
    }
    if let CrateType::Executable = crate_type
        && !sess.target.executables
    {
        return true;
    }

    false
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=categorize_crate_type | COMPLEXITY=3 | LINES=15

```rust
pub const CRATE_TYPES: &[(Symbol, CrateType)] = &[
    (sym::rlib, CrateType::Rlib),
    (sym::dylib, CrateType::Dylib),
    (sym::cdylib, CrateType::Cdylib),
    (sym::lib, config::default_lib_output()),
    (sym::staticlib, CrateType::Staticlib),
    (sym::proc_dash_macro, CrateType::ProcMacro),
    (sym::bin, CrateType::Executable),
    (sym::sdylib, CrateType::Sdylib),
];

pub fn categorize_crate_type(s: Symbol) -> Option<CrateType> {
    Some(CRATE_TYPES.iter().find(|(key, _)| *key == s)?.1)
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=collect_crate_types | COMPLEXITY=35 | LINES=59

```rust
pub fn collect_crate_types(session: &Session, attrs: &[ast::Attribute]) -> Vec<CrateType> {
    // If we're generating a test executable, then ignore all other output
    // styles at all other locations
    if session.opts.test {
        if !session.target.executables {
            session.dcx().emit_warn(errors::UnsupportedCrateTypeForTarget {
                crate_type: CrateType::Executable,
                target_triple: &session.opts.target_triple,
            });
            return Vec::new();
        }
        return vec![CrateType::Executable];
    }

    // Shadow `sdylib` crate type in interface build.
    if session.opts.unstable_opts.build_sdylib_interface {
        return vec![CrateType::Rlib];
    }

    // Only check command line flags if present. If no types are specified by
    // command line, then reuse the empty `base` Vec to hold the types that
    // will be found in crate attributes.
    // JUSTIFICATION: before wrapper fn is available
    #[allow(rustc::bad_opt_access)]
    let mut base = session.opts.crate_types.clone();
    if base.is_empty() {
        let attr_types = attrs.iter().filter_map(|a| {
            if a.has_name(sym::crate_type)
                && let Some(s) = a.value_str()
            {
                categorize_crate_type(s)
            } else {
                None
            }
        });
        base.extend(attr_types);
        if base.is_empty() {
            base.push(default_output_for_target(session));
        } else {
            base.sort();
            base.dedup();
        }
    }

    base.retain(|crate_type| {
        if invalid_output_for_target(session, *crate_type) {
            session.dcx().emit_warn(errors::UnsupportedCrateTypeForTarget {
                crate_type: *crate_type,
                target_triple: &session.opts.target_triple,
            });
            false
        } else {
            true
        }
    });

    base
}
```

---
*Generated by AST tracing system*
