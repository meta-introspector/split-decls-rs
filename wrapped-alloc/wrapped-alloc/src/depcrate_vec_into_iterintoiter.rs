// Generated macro for IntoIter (struct)
macro_rules! Depcrate_vec_into_iterIntoIter {
() => {
// Module: crate::vec::into_iter
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves out of a vector."] # [doc = ""] # [doc = " This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)"] # [doc = " (provided by the [`IntoIterator`] trait)."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " let v = vec![0, 1, 2];"] # [doc = " let iter: std::vec::IntoIter<_> = v.into_iter();"] # [doc = " ```"] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_insignificant_dtor] pub struct IntoIter < T , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > { pub (super) buf : NonNull < T > , pub (super) phantom : PhantomData < T > , pub (super) cap : usize , pub (super) alloc : ManuallyDrop < A > , pub (super) ptr : NonNull < T > , # [doc = " If T is a ZST, this is actually ptr+len. This encoding is picked so that"] # [doc = " ptr == end is a quick test for the Iterator being empty, that works"] # [doc = " for both ZST and non-ZST."] # [doc = " For non-ZSTs the pointer is treated as `NonNull<T>`"] pub (super) end : * const T , }
};
}
