// Generated macro for EmulateItemResult (enum)
macro_rules! Depcrate_shimsEmulateItemResult {
() => {
// Module: crate::shims
// Provides: {"EmulateItemResult"}
// Dependencies: {}
# [doc = " What needs to be done after emulating an item (a shim or an intrinsic) is done."] pub enum EmulateItemResult { # [doc = " The caller is expected to jump to the return block."] NeedsReturn , # [doc = " The caller is expected to jump to the unwind block."] NeedsUnwind , # [doc = " Jumping to the next block has already been taken care of."] AlreadyJumped , # [doc = " The item is not supported."] NotSupported , }
};
}
