// Generated macro for impl_312 (impl)
macro_rules! Depcrate_extensionsimpl_312 {
() => {
// Module: crate::extensions
// Provides: {"impl_312"}
// Dependencies: {}
impl Extensions { # [doc = " Creates an extension flag from an ident."] # [must_use] pub fn from_ident (ident : & str) -> Option < Extensions > { for (name , extension) in Extensions :: all () . iter_names () { if ident == name . to_lowercase () { return Some (extension) ; } } None } }
};
}
