// Generated macro for to_normal_path_components (module)
macro_rules! Depcrate_stackto_normal_path_components {
() => {
// Module: crate::stack
// Provides: {"to_normal_path_components"}
// Dependencies: {}
# [doc = ""] pub mod to_normal_path_components { use std :: path :: PathBuf ; # [doc = " The error used in [`ToNormalPathComponents::to_normal_path_components()`](super::ToNormalPathComponents::to_normal_path_components())."] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Input path \"{path}\" contains relative or absolute components" , path = . 0 . display ())] NotANormalComponent (PathBuf) , # [error ("Could not convert to UTF8 or from UTF8 due to ill-formed input")] IllegalUtf8 , } }
};
}
