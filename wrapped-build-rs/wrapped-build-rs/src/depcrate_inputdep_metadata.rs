// Generated macro for dep_metadata (function)
macro_rules! Depcrate_inputdep_metadata {
() => {
// Module: crate::input
// Provides: {"dep_metadata"}
// Dependencies: {}
# [doc = " [Metadata] set by dependencies. For more information, see build script"] # [doc = " documentation about [the `links` manifest key][links]."] # [doc = ""] # [doc = " [metadata]: crate::output::metadata"] # [doc = " [links]: https://doc.rust-lang.org/stable/cargo/reference/build-scripts.html#the-links-manifest-key"] # [track_caller] pub fn dep_metadata (name : & str , key : & str) -> Option < String > { if ! is_crate_name (name) { panic ! ("invalid dependency name {name:?}") } if ! is_ascii_ident (key) { panic ! ("invalid metadata key {key:?}") } let name = name . to_uppercase () . replace ('-' , "_") ; let key = key . to_uppercase () . replace ('-' , "_") ; let key = format ! ("DEP_{name}_{key}") ; ENV . get (& key) . map (to_string) }
};
}
