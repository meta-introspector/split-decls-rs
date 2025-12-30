// Generated macro for impl_521 (impl)
macro_rules! Depcrate_vectorimpl_521 {
() => {
// Module: crate::vector
// Provides: {"impl_521"}
// Dependencies: {}
# [cfg (not (has_specialisation))] impl < A : Clone + PartialEq > PartialEq for Vector < A > { fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
