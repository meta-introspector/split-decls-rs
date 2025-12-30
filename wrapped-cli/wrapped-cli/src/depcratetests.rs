// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: path :: PathBuf ; use crate :: add_extension ; # [test] fn extension_added () { let filename = PathBuf :: from ("README.md") ; assert_eq ! (add_extension (& filename , ".zst") , PathBuf :: from ("README.md.zst")) ; } }
};
}
