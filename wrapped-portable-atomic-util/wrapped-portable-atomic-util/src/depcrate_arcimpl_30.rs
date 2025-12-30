// Generated macro for impl_30 (impl)
macro_rules! Depcrate_arcimpl_30 {
() => {
// Module: crate::arc
// Provides: {"impl_30"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_alloc_layout_extras))] impl < T > Arc < [T] > { # [doc = " Constructs a new atomically reference-counted slice with uninitialized contents."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use portable_atomic_util::Arc;"] # [doc = ""] # [doc = " let mut values = Arc::<[u32]>::new_uninit_slice(3);"] # [doc = ""] # [doc = " // Deferred initialization:"] # [doc = " let data = Arc::get_mut(&mut values).unwrap();"] # [doc = " data[0].write(1);"] # [doc = " data[1].write(2);"] # [doc = " data[2].write(3);"] # [doc = ""] # [doc = " let values = unsafe { values.assume_init() };"] # [doc = ""] # [doc = " assert_eq!(*values, [1, 2, 3])"] # [doc = " ```"] # [inline] # [must_use] pub fn new_uninit_slice (len : usize) -> Arc < [mem :: MaybeUninit < T >] > { unsafe { Arc :: from_ptr (Arc :: allocate_for_slice (len)) } } }
};
}
