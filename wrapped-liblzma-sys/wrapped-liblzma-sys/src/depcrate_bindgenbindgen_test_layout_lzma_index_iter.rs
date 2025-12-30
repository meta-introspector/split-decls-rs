// Generated macro for bindgen_test_layout_lzma_index_iter (function)
macro_rules! Depcrate_bindgenbindgen_test_layout_lzma_index_iter {
() => {
// Module: crate::bindgen
// Provides: {"bindgen_test_layout_lzma_index_iter"}
// Dependencies: {}
# [test] fn bindgen_test_layout_lzma_index_iter () { const UNINIT : :: std :: mem :: MaybeUninit < lzma_index_iter > = :: std :: mem :: MaybeUninit :: uninit () ; let ptr = UNINIT . as_ptr () ; assert_eq ! (:: std :: mem :: size_of ::< lzma_index_iter > () , 304usize , "Size of lzma_index_iter") ; assert_eq ! (:: std :: mem :: align_of ::< lzma_index_iter > () , 8usize , "Alignment of lzma_index_iter") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . stream) as usize - ptr as usize } , 0usize , "Offset of field: lzma_index_iter::stream") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . block) as usize - ptr as usize } , 120usize , "Offset of field: lzma_index_iter::block") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . internal) as usize - ptr as usize } , 256usize , "Offset of field: lzma_index_iter::internal") ; }
};
}
