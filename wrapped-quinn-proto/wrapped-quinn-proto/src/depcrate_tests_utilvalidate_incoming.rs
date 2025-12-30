// Generated macro for validate_incoming (function)
macro_rules! Depcrate_tests_utilvalidate_incoming {
() => {
// Module: crate::tests::util
// Provides: {"validate_incoming"}
// Dependencies: {}
pub (super) fn validate_incoming (incoming : & Incoming) -> IncomingConnectionBehavior { if incoming . remote_address_validated () { IncomingConnectionBehavior :: Accept } else { IncomingConnectionBehavior :: Retry } }
};
}
