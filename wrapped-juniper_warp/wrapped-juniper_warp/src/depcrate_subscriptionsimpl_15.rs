// Generated macro for impl_15 (impl)
macro_rules! Depcrate_subscriptionsimpl_15 {
() => {
// Module: crate::subscriptions
// Provides: {"impl_15"}
// Dependencies: {}
impl < S : ScalarValue > TryFrom < Message > for graphql_ws :: ClientMessage < S > { type Error = serde_json :: Error ; fn try_from (msg : Message) -> serde_json :: Result < Self > { if msg . 0 . is_close () { Ok (Self :: ConnectionTerminate) } else { serde_json :: from_slice (msg . 0 . as_bytes ()) } } }
};
}
