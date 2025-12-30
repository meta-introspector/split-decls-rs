// Generated macro for impl_61 (impl)
macro_rules! Depcrateimpl_61 {
() => {
// Module: crate
// Provides: {"impl_61"}
// Dependencies: {}
impl < T > KeySizeUser for T where T : InnerUser , T :: Inner : KeySizeUser , { type KeySize = < T :: Inner as KeySizeUser > :: KeySize ; }
};
}
