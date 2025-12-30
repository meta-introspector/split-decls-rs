// Generated macro for acquire (macro)
macro_rules! Depcrate_arcacquire {
() => {
// Module: crate::arc
// Provides: {"acquire"}
// Dependencies: {}
# [cfg (portable_atomic_sanitize_thread)] macro_rules ! acquire { ($ x : expr) => { $ x . load (Acquire) } ; }
};
}
