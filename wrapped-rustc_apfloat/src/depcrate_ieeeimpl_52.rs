// Generated macro for impl_52 (impl)
macro_rules! Depcrate_ieeeimpl_52 {
() => {
// Module: crate::ieee
// Provides: {"impl_52"}
// Dependencies: {}
impl < S : Semantics > PartialEq for IeeeFloat < S > { fn eq (& self , rhs : & Self) -> bool { self . partial_cmp (rhs) == Some (Ordering :: Equal) } }
};
}
