// Generated macro for impl_62 (impl)
macro_rules! Depcrate_arcimpl_62 {
() => {
// Module: crate::arc
// Provides: {"impl_62"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_min_const_generics))] impl Default for Arc < str > { # [doc = " Creates an empty str inside an Arc."] # [doc = ""] # [doc = " This may or may not share an allocation with other Arcs."] # [inline] fn default () -> Self { let arc : Arc < [u8] > = Arc :: default () ; debug_assert ! (core :: str :: from_utf8 (& arc) . is_ok ()) ; let ptr = Arc :: into_inner_non_null (arc) ; unsafe { Arc :: from_ptr (ptr . as_ptr () as * mut ArcInner < str >) } } }
};
}
