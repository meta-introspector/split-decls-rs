// Generated macro for exchange_malloc (function)
macro_rules! Depcrate_allocexchange_malloc {
() => {
// Module: crate::alloc
// Provides: {"exchange_malloc"}
// Dependencies: {}
# [doc = " The allocator for `Box`."] # [cfg (not (no_global_oom_handling))] # [lang = "exchange_malloc"] # [inline] # [cfg_attr (miri , track_caller)] unsafe fn exchange_malloc (size : usize , align : usize) -> * mut u8 { let layout = unsafe { Layout :: from_size_align_unchecked (size , align) } ; match Global . allocate (layout) { Ok (ptr) => ptr . as_mut_ptr () , Err (_) => handle_alloc_error (layout) , } }
};
}
