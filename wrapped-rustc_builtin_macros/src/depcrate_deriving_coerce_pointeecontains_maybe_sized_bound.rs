// Generated macro for contains_maybe_sized_bound (function)
macro_rules! Depcrate_deriving_coerce_pointeecontains_maybe_sized_bound {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"contains_maybe_sized_bound"}
// Dependencies: {}
fn contains_maybe_sized_bound (bounds : & [GenericBound]) -> bool { bounds . iter () . any (is_maybe_sized_bound) }
};
}
