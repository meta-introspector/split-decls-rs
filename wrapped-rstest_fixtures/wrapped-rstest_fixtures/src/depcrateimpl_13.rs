// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < F : FnOnce () -> () > From < F > for TearDownClosure < F > { fn from (closure : F) -> Self { TearDownClosure (closure) } }
};
}
