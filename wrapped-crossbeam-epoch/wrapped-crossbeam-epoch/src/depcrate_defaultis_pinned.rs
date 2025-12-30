// Generated macro for is_pinned (function)
macro_rules! Depcrate_defaultis_pinned {
() => {
// Module: crate::default
// Provides: {"is_pinned"}
// Dependencies: {}
# [doc = " Returns `true` if the current thread is pinned."] # [inline] pub fn is_pinned () -> bool { with_handle (| handle | handle . is_pinned ()) }
};
}
