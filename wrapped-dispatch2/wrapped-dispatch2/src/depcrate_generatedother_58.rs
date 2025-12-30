// Generated macro for other_58 (other)
macro_rules! Depcrate_generatedother_58 {
() => {
// Module: crate::generated
// Provides: {"other_58"}
// Dependencies: {}
extern "C" { # [doc = " Create, synchronously execute and release a dispatch block object from the"] # [doc = " specified block and flags."] # [doc = ""] # [doc = ""] # [doc = " Behaves identically to the sequence"] # [doc = " <code>"] # [doc = " dispatch_block_t b = dispatch_block_create(flags, block);"] # [doc = " b();"] # [doc = " Block_release(b);"] # [doc = " </code>"] # [doc = " but may be implemented more efficiently internally by not requiring a copy"] # [doc = " to the heap of the specified block or the allocation of a new block object."] # [doc = ""] # [doc = ""] # [doc = " Parameter `flags`: Configuration flags for the temporary block object."] # [doc = " The result of passing a value that is not a bitwise OR of flags from"] # [doc = " dispatch_block_flags_t is undefined."] # [doc = ""] # [doc = ""] # [doc = " Parameter `block`: The block to create the temporary block object from."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `block` must be a valid pointer."] # [cfg (feature = "block2")] pub fn dispatch_block_perform (flags : dispatch_block_flags_t , block : dispatch_block_t) ; }
};
}
