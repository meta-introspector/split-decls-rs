// Generated macro for impl_642 (impl)
macro_rules! Depcrate_base_array_storageimpl_642 {
() => {
// Module: crate::base::array_storage
// Provides: {"impl_642"}
// Dependencies: {}
impl < T , const R : usize , const C : usize > ArrayStorage < T , R , C > { # [doc = " Converts this array storage to a slice."] # [inline] pub fn as_slice (& self) -> & [T] { unsafe { self . as_slice_unchecked () } } # [doc = " Converts this array storage to a mutable slice."] # [inline] pub fn as_mut_slice (& mut self) -> & mut [T] { unsafe { self . as_mut_slice_unchecked () } } }
};
}
