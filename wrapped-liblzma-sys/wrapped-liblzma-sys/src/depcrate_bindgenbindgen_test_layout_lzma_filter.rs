// Generated macro for bindgen_test_layout_lzma_filter (function)
macro_rules! Depcrate_bindgenbindgen_test_layout_lzma_filter {
() => {
// Module: crate::bindgen
// Provides: {"bindgen_test_layout_lzma_filter"}
// Dependencies: {}
# [test] fn bindgen_test_layout_lzma_filter () { const UNINIT : :: std :: mem :: MaybeUninit < lzma_filter > = :: std :: mem :: MaybeUninit :: uninit () ; let ptr = UNINIT . as_ptr () ; assert_eq ! (:: std :: mem :: size_of ::< lzma_filter > () , 16usize , "Size of lzma_filter") ; assert_eq ! (:: std :: mem :: align_of ::< lzma_filter > () , 8usize , "Alignment of lzma_filter") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . id) as usize - ptr as usize } , 0usize , "Offset of field: lzma_filter::id") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . options) as usize - ptr as usize } , 8usize , "Offset of field: lzma_filter::options") ; }
};
}
