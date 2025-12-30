// Generated macro for impl_54 (impl)
macro_rules! Depcrate_serimpl_54 {
() => {
// Module: crate::ser
// Provides: {"impl_54"}
// Dependencies: {}
impl < W : Write > From < W > for Serializer < W > { # [inline] fn from (writer : W) -> Self { Self (writer . into ()) } }
};
}
