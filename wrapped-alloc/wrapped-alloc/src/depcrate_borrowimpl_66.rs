// Generated macro for impl_66 (impl)
macro_rules! Depcrate_borrowimpl_66 {
() => {
// Module: crate::borrow
// Provides: {"impl_66"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < 'a , B : ? Sized + ToOwned > Borrow < B > for Cow < 'a , B > { fn borrow (& self) -> & B { & * * self } }
};
}
