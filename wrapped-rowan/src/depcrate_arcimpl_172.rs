// Generated macro for impl_172 (impl)
macro_rules! Depcrate_arcimpl_172 {
() => {
// Module: crate::arc
// Provides: {"impl_172"}
// Dependencies: {}
impl < H , T > Clone for ThinArc < H , T > { # [inline] fn clone (& self) -> Self { ThinArc :: with_arc (self , | a | Arc :: into_thin (a . clone ())) } }
};
}
