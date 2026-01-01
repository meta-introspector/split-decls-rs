// SRC: ../rust/compiler/rustc_codegen_cranelift/build_system/path.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) struct Dirs {
    pub(crate) source_dir: PathBuf,
    pub(crate) download_dir: PathBuf,
    pub(crate) build_dir: PathBuf,
    pub(crate) dist_dir: PathBuf,
    pub(crate) frozen: bool,
}
/* AST_META: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
enum PathBase {
    Source,
    Build,
}
/* AST_META: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Debug, Copy, Clone)]
pub(crate) struct RelPath {
    base: PathBase,
    suffix: &'static str,
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=17 */

impl RelPath {
    pub(crate) const fn source(suffix: &'static str) -> RelPath {
        RelPath { base: PathBase::Source, suffix }
    }

    pub(crate) const fn build(suffix: &'static str) -> RelPath {
        RelPath { base: PathBase::Build, suffix }
    }

    pub(crate) fn to_path(&self, dirs: &Dirs) -> PathBuf {
        match self.base {
            PathBase::Source => dirs.source_dir.join(self.suffix),
            PathBase::Build => dirs.build_dir.join(self.suffix),
        }
    }
}