// Generated macro for impl_522 (impl)
macro_rules! Depcrate_vectorimpl_522 {
() => {
// Module: crate::vector
// Provides: {"impl_522"}
// Dependencies: {}
# [cfg (has_specialisation)] impl < A : Clone + PartialEq > PartialEq for Vector < A > { default fn eq (& self , other : & Self) -> bool { self . len () == other . len () && self . iter () . eq (other . iter ()) } }
};
}
