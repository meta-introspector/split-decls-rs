// Generated macro for PushTransferProgress (type)
macro_rules! Depcrate_remote_callbacksPushTransferProgress {
() => {
// Module: crate::remote_callbacks
// Provides: {"PushTransferProgress"}
// Dependencies: {}
# [doc = " Callback for push transfer progress"] # [doc = ""] # [doc = " Parameters:"] # [doc = " * current"] # [doc = " * total"] # [doc = " * bytes"] pub type PushTransferProgress < 'a > = dyn FnMut (usize , usize , usize) + 'a ;
};
}
