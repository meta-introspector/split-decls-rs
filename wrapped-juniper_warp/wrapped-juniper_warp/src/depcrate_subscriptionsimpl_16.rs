// Generated macro for impl_16 (impl)
macro_rules! Depcrate_subscriptionsimpl_16 {
() => {
// Module: crate::subscriptions
// Provides: {"impl_16"}
// Dependencies: {}
impl < S : ScalarValue > TryFrom < Message > for graphql_transport_ws :: Input < S > { type Error = serde_json :: Error ; fn try_from (msg : Message) -> serde_json :: Result < Self > { if msg . 0 . is_close () { Ok (Self :: Close) } else { serde_json :: from_slice (msg . 0 . as_bytes ()) . map (Self :: Message) } } }
};
}
