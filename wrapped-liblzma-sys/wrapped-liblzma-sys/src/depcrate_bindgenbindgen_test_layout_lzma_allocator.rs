// Generated macro for bindgen_test_layout_lzma_allocator (function)
macro_rules! Depcrate_bindgenbindgen_test_layout_lzma_allocator {
() => {
// Module: crate::bindgen
// Provides: {"bindgen_test_layout_lzma_allocator"}
// Dependencies: {}
# [test] fn bindgen_test_layout_lzma_allocator () { const UNINIT : :: std :: mem :: MaybeUninit < lzma_allocator > = :: std :: mem :: MaybeUninit :: uninit () ; let ptr = UNINIT . as_ptr () ; assert_eq ! (:: std :: mem :: size_of ::< lzma_allocator > () , 24usize , "Size of lzma_allocator") ; assert_eq ! (:: std :: mem :: align_of ::< lzma_allocator > () , 8usize , "Alignment of lzma_allocator") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . alloc) as usize - ptr as usize } , 0usize , "Offset of field: lzma_allocator::alloc") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . free) as usize - ptr as usize } , 8usize , "Offset of field: lzma_allocator::free") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . opaque) as usize - ptr as usize } , 16usize , "Offset of field: lzma_allocator::opaque") ; }
};
}
