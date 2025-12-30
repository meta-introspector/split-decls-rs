// Generated macro for cabi_realloc (function)
macro_rules! Depcrate_rtcabi_realloc {
() => {
// Module: crate::rt
// Provides: {"cabi_realloc"}
// Dependencies: {}
# [doc = " NB: this function is called by a generated function in the"] # [doc = " `cabi_realloc` module above. It's otherwise never explicitly called."] # [doc = ""] # [doc = " For more information about this see `./ci/rebuild-libwit-bindgen-cabi.sh`."] # [cfg (not (target_env = "p2"))] pub unsafe fn cabi_realloc (old_ptr : * mut u8 , old_len : usize , align : usize , new_len : usize ,) -> * mut u8 { use alloc :: alloc :: { alloc as allocate , handle_alloc_error , realloc , Layout } ; let layout ; let ptr = if old_len == 0 { if new_len == 0 { return align as * mut u8 ; } layout = Layout :: from_size_align_unchecked (new_len , align) ; allocate (layout) } else { debug_assert_ne ! (new_len , 0 , "non-zero old_len requires non-zero new_len!") ; layout = Layout :: from_size_align_unchecked (old_len , align) ; realloc (old_ptr , layout , new_len) } ; if ptr . is_null () { if cfg ! (debug_assertions) { handle_alloc_error (layout) ; } else { # [cfg (target_arch = "wasm32")] core :: arch :: wasm32 :: unreachable () ; # [cfg (not (target_arch = "wasm32"))] unreachable ! () ; } } return ptr ; }
};
}
