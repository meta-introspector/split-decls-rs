// Generated macro for impl_47 (impl)
macro_rules! Depcrate_combinators_fuseimpl_47 {
() => {
// Module: crate::combinators::fuse
// Provides: {"impl_47"}
// Dependencies: {}
impl < B > Fuse < B > where B : Body , { # [doc = " Returns a fused body."] pub fn new (body : B) -> Self { Self { inner : if body . is_end_stream () { None } else { Some (body) } , } } }
};
}
