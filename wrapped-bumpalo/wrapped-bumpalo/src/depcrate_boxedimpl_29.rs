// Generated macro for impl_29 (impl)
macro_rules! Depcrate_boxedimpl_29 {
() => {
// Module: crate::boxed
// Provides: {"impl_29"}
// Dependencies: {}
impl < 'a , T : ? Sized > borrow :: Borrow < T > for Box < 'a , T > { fn borrow (& self) -> & T { & * * self } }
};
}
