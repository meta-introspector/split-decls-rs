// Generated macro for macro_141 (macro)
macro_rules! Depcrate_syncmacro_141 {
() => {
// Module: crate::sync
// Provides: {"macro_141"}
// Dependencies: {}
switch_alloc_default ! { # [doc = " Thread-local proxy for [`SyncBlinkAlloc`]."] # [doc = ""] # [doc = " Using proxy can yield better performance when"] # [doc = " it is possible to create proxy once to use for many allocations."] # [doc = ""] # [doc = " See [`SyncBlinkAlloc::local`] for more details."] pub struct LocalBlinkAlloc <'a , A : Allocator = + Global > { arena : ArenaLocal , shared : &'a SyncBlinkAlloc < A >, } }
};
}
