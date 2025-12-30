// Generated macro for remove_file (function)
macro_rules! Depcrate_utilsremove_file {
() => {
// Module: crate::utils
// Provides: {"remove_file"}
// Dependencies: {}
pub fn remove_file < P : AsRef < Path > + ? Sized > (file_path : & P) -> Result < () , String > { std :: fs :: remove_file (file_path) . map_err (| error | { format ! ("Failed to remove `{}`: {:?}" , file_path . as_ref () . display () , error) }) }
};
}
