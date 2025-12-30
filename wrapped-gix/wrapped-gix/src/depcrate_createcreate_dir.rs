// Generated macro for create_dir (function)
macro_rules! Depcrate_createcreate_dir {
() => {
// Module: crate::create
// Provides: {"create_dir"}
// Dependencies: {}
fn create_dir (p : & Path) -> Result < () , Error > { fs :: create_dir_all (p) . map_err (| e | Error :: CreateDirectory { source : e , path : p . to_owned () , }) }
};
}
