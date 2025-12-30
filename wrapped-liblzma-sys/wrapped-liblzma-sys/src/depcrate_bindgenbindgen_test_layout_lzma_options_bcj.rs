// Generated macro for bindgen_test_layout_lzma_options_bcj (function)
macro_rules! Depcrate_bindgenbindgen_test_layout_lzma_options_bcj {
() => {
// Module: crate::bindgen
// Provides: {"bindgen_test_layout_lzma_options_bcj"}
// Dependencies: {}
# [test] fn bindgen_test_layout_lzma_options_bcj () { const UNINIT : :: std :: mem :: MaybeUninit < lzma_options_bcj > = :: std :: mem :: MaybeUninit :: uninit () ; let ptr = UNINIT . as_ptr () ; assert_eq ! (:: std :: mem :: size_of ::< lzma_options_bcj > () , 4usize , "Size of lzma_options_bcj") ; assert_eq ! (:: std :: mem :: align_of ::< lzma_options_bcj > () , 4usize , "Alignment of lzma_options_bcj") ; assert_eq ! (unsafe { :: std :: ptr :: addr_of ! ((* ptr) . start_offset) as usize - ptr as usize } , 0usize , "Offset of field: lzma_options_bcj::start_offset") ; }
};
}
