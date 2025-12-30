// Generated macro for impl_1618 (impl)
macro_rules! Depcrate_syncimpl_1618 {
() => {
// Module: crate::sync
// Provides: {"impl_1618"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "more_rc_default_impls" , since = "1.80.0")] impl Default for Arc < str > { # [doc = " Creates an empty str inside an Arc"] # [doc = ""] # [doc = " This may or may not share an allocation with other Arcs."] # [inline] fn default () -> Self { let arc : Arc < [u8] > = Default :: default () ; debug_assert ! (core :: str :: from_utf8 (&* arc) . is_ok ()) ; let (ptr , alloc) = Arc :: into_inner_with_allocator (arc) ; unsafe { Arc :: from_ptr_in (ptr . as_ptr () as * mut ArcInner < str > , alloc) } } }
};
}
