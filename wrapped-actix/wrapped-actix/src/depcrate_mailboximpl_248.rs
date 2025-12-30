// Generated macro for impl_248 (impl)
macro_rules! Depcrate_mailboximpl_248 {
() => {
// Module: crate::mailbox
// Provides: {"impl_248"}
// Dependencies: {}
impl < A > Default for Mailbox < A > where A : Actor , A :: Context : AsyncContext < A > , { # [inline] fn default () -> Self { let (_ , rx) = channel :: channel (DEFAULT_CAPACITY) ; Mailbox { msgs : rx } } }
};
}
