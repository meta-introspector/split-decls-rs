// Generated macro for impl_61 (impl)
macro_rules! Depcrate_frontend_serdeimpl_61 {
() => {
// Module: crate::frontend::serde
// Provides: {"impl_61"}
// Dependencies: {}
impl < B : PatternBackend > Clone for PatternString < B > where Box < B :: Store > : for < 'a > From < & 'a B :: Store > , { fn clone (& self) -> Self { Self (self . 0 . clone ()) } }
};
}
