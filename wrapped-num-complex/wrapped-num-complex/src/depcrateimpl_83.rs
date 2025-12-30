// Generated macro for impl_83 (impl)
macro_rules! Depcrateimpl_83 {
() => {
// Module: crate
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a , T : Clone + Num > From < & 'a T > for Complex < T > { # [inline] fn from (re : & T) -> Self { From :: from (re . clone ()) } }
};
}
