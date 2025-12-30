// Generated macro for impl_231 (impl)
macro_rules! Depcrate_positionimpl_231 {
() => {
// Module: crate::position
// Provides: {"impl_231"}
// Dependencies: {}
# [allow (clippy :: non_canonical_partial_ord_impl)] impl < 'i > PartialOrd for Position < 'i > { fn partial_cmp (& self , other : & Position < 'i >) -> Option < Ordering > { if ptr :: eq (self . input , other . input) { self . pos . partial_cmp (& other . pos) } else { None } } }
};
}
