// Generated macro for impl_43 (impl)
macro_rules! Depcrate_arena_syncimpl_43 {
() => {
// Module: crate::arena::sync
// Provides: {"impl_43"}
// Dependencies: {}
impl Drop for ArenaSync { # [inline (always)] fn drop (& mut self) { debug_assert ! (self . inner . get_mut () . root . is_none () , "Owner must reset `ArenaSync` with `keep_last` set to `false` before drop") ; } }
};
}
