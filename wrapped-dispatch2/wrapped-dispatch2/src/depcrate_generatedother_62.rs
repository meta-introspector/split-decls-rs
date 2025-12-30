// Generated macro for other_62 (other)
macro_rules! Depcrate_generatedother_62 {
() => {
// Module: crate::generated
// Provides: {"other_62"}
// Dependencies: {}
extern "C" { # [doc = " Tests whether the given dispatch block object has been canceled."] # [doc = ""] # [doc = ""] # [doc = " Parameter `block`: The dispatch block object to test."] # [doc = " The result of passing NULL or a block object not returned by one of the"] # [doc = " dispatch_block_create* functions is undefined."] # [doc = ""] # [doc = ""] # [doc = " Returns: Non-zero if canceled and zero if not canceled."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `block` must be a valid pointer."] # [cfg (feature = "block2")] # [must_use] pub fn dispatch_block_testcancel (block : dispatch_block_t) -> isize ; }
};
}
