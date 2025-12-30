// Generated macro for other_61 (other)
macro_rules! Depcrate_generatedother_61 {
() => {
// Module: crate::generated
// Provides: {"other_61"}
// Dependencies: {}
extern "C" { # [doc = " Asynchronously cancel the specified dispatch block object."] # [doc = ""] # [doc = ""] # [doc = " Cancellation causes any future execution of the dispatch block object to"] # [doc = " return immediately, but does not affect any execution of the block object"] # [doc = " that is already in progress."] # [doc = ""] # [doc = " Release of any resources associated with the block object will be delayed"] # [doc = " until execution of the block object is next attempted (or any execution"] # [doc = " already in progress completes)."] # [doc = ""] # [doc = " NOTE: care needs to be taken to ensure that a block object that may be"] # [doc = " canceled does not capture any resources that require execution of the"] # [doc = " block body in order to be released (e.g. memory allocated with"] # [doc = " malloc(3) that the block body calls free(3) on). Such resources will"] # [doc = " be leaked if the block body is never executed due to cancellation."] # [doc = ""] # [doc = ""] # [doc = " Parameter `block`: The dispatch block object to cancel."] # [doc = " The result of passing NULL or a block object not returned by one of the"] # [doc = " dispatch_block_create* functions is undefined."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `block` must be a valid pointer."] # [cfg (feature = "block2")] pub fn dispatch_block_cancel (block : dispatch_block_t) ; }
};
}
