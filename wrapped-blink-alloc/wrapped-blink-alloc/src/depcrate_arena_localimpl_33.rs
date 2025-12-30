// Generated macro for impl_33 (impl)
macro_rules! Depcrate_arena_localimpl_33 {
() => {
// Module: crate::arena::local
// Provides: {"impl_33"}
// Dependencies: {}
impl Drop for ArenaLocal { # [inline (always)] fn drop (& mut self) { debug_assert ! (self . root . get () . is_none () , "Owner must reset `ArenaLocal` with `keep_last` set to `false` before drop") ; } }
};
}
