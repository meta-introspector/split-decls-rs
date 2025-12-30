// Generated macro for impl_63 (impl)
macro_rules! Depcrate_arcimpl_63 {
() => {
// Module: crate::arc
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_min_const_generics))] impl < T > Default for Arc < [T] > { # [doc = " Creates an empty `[T]` inside an Arc."] # [doc = ""] # [doc = " This may or may not share an allocation with other Arcs."] # [inline] fn default () -> Self { let arr : [T ; 0] = [] ; Arc :: from (arr) } }
};
}
