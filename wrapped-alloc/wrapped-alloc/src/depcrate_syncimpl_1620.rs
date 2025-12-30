// Generated macro for impl_1620 (impl)
macro_rules! Depcrate_syncimpl_1620 {
() => {
// Module: crate::sync
// Provides: {"impl_1620"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "more_rc_default_impls" , since = "1.80.0")] impl < T > Default for Arc < [T] > { # [doc = " Creates an empty `[T]` inside an Arc"] # [doc = ""] # [doc = " This may or may not share an allocation with other Arcs."] # [inline] fn default () -> Self { if align_of :: < T > () <= MAX_STATIC_INNER_SLICE_ALIGNMENT { let inner : NonNull < SliceArcInnerForStatic > = NonNull :: from (& STATIC_INNER_SLICE) ; let inner : NonNull < ArcInner < [T ; 0] > > = inner . cast () ; let this : mem :: ManuallyDrop < Arc < [T ; 0] > > = unsafe { mem :: ManuallyDrop :: new (Arc :: from_inner (inner)) } ; return (* this) . clone () ; } let arr : [T ; 0] = [] ; Arc :: from (arr) } }
};
}
