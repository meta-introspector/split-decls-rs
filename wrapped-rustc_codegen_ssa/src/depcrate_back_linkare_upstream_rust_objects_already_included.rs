// Generated macro for are_upstream_rust_objects_already_included (function)
macro_rules! Depcrate_back_linkare_upstream_rust_objects_already_included {
() => {
// Module: crate::back::link
// Provides: {"are_upstream_rust_objects_already_included"}
// Dependencies: {}
pub (crate) fn are_upstream_rust_objects_already_included (sess : & Session) -> bool { match sess . lto () { config :: Lto :: Fat => true , config :: Lto :: Thin => { ! sess . opts . cg . linker_plugin_lto . enabled () } config :: Lto :: No | config :: Lto :: ThinLocal => false , } }
};
}
