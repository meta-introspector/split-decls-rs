// Generated macro for impl_68 (impl)
macro_rules! Depcrate_borrowimpl_68 {
() => {
// Module: crate::borrow
// Provides: {"impl_68"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T > ToOwned for T where T : Clone , { type Owned = T ; fn to_owned (& self) -> T { self . clone () } fn clone_into (& self , target : & mut T) { target . clone_from (self) ; } }
};
}
