// Generated macro for impl_1781 (impl)
macro_rules! Depcrate_vec_into_iterimpl_1781 {
() => {
// Module: crate::vec::into_iter
// Provides: {"impl_1781"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > ExactSizeIterator for IntoIter < T , A > { fn is_empty (& self) -> bool { if T :: IS_ZST { self . ptr . as_ptr () == self . end as * mut _ } else { self . ptr == non_null ! (self . end , T) } } }
};
}
