// Generated macro for impl_70 (impl)
macro_rules! Depcrate_locationimpl_70 {
() => {
// Module: crate::location
// Provides: {"impl_70"}
// Dependencies: {}
impl PartialEq for Location { fn eq (& self , rhs : & Self) -> bool { if let Some (lhs) = self . id () { if let Some (rhs) = rhs . id () { return lhs == rhs ; } } false } }
};
}
