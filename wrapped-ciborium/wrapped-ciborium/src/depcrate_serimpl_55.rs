// Generated macro for impl_55 (impl)
macro_rules! Depcrate_serimpl_55 {
() => {
// Module: crate::ser
// Provides: {"impl_55"}
// Dependencies: {}
impl < W : Write > From < Encoder < W > > for Serializer < W > { # [inline] fn from (writer : Encoder < W >) -> Self { Self (writer) } }
};
}
