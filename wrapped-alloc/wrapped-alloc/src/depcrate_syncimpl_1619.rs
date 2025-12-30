// Generated macro for impl_1619 (impl)
macro_rules! Depcrate_syncimpl_1619 {
() => {
// Module: crate::sync
// Provides: {"impl_1619"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "more_rc_default_impls" , since = "1.80.0")] impl Default for Arc < core :: ffi :: CStr > { # [doc = " Creates an empty CStr inside an Arc"] # [doc = ""] # [doc = " This may or may not share an allocation with other Arcs."] # [inline] fn default () -> Self { use core :: ffi :: CStr ; let inner : NonNull < ArcInner < [u8] > > = NonNull :: from (& STATIC_INNER_SLICE . inner) ; let inner : NonNull < ArcInner < CStr > > = NonNull :: new (inner . as_ptr () as * mut ArcInner < CStr >) . unwrap () ; let this : mem :: ManuallyDrop < Arc < CStr > > = unsafe { mem :: ManuallyDrop :: new (Arc :: from_inner (inner)) } ; (* this) . clone () } }
};
}
