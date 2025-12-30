// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
# [doc = " A reference containing just constant parts is safe to clone."] impl < SomePart , Reference : Copy + HasTarget > Clone for Const < SomePart , Reference > { # [inline (always)] fn clone (& self) -> Self { * self } }
};
}
