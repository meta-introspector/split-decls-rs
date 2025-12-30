// Generated macro for create_dir (function)
macro_rules! Depcrate_utilscreate_dir {
() => {
// Module: crate::utils
// Provides: {"create_dir"}
// Dependencies: {}
pub fn create_dir < P : AsRef < Path > > (path : P) -> Result < () , String > { fs :: create_dir_all (& path) . map_err (| error | { format ! ("Failed to create directory `{}`: {:?}" , path . as_ref () . display () , error) }) }
};
}
