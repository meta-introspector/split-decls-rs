// Generated macro for acquire (macro)
macro_rules! Depcrate_syncacquire {
() => {
// Module: crate::sync
// Provides: {"acquire"}
// Dependencies: {}
# [cfg (sanitize = "thread")] macro_rules ! acquire { ($ x : expr) => { $ x . load (Acquire) } ; }
};
}
