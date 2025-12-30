// Generated macro for impl_30 (impl)
macro_rules! Depcrate_subscriptionsimpl_30 {
() => {
// Module: crate::subscriptions
// Provides: {"impl_30"}
// Dependencies: {}
impl < S : ScalarValue > TryFrom < Message > for graphql_transport_ws :: Input < S > { type Error = Error ; fn try_from (msg : Message) -> Result < Self , Self :: Error > { match msg . 0 { ws :: Message :: Text (text) => serde_json :: from_slice (text . as_bytes ()) . map (Self :: Message) . map_err (Error :: Serde) , ws :: Message :: Binary (bytes) => serde_json :: from_slice (bytes . as_ref ()) . map (Self :: Message) . map_err (Error :: Serde) , ws :: Message :: Close (_) => Ok (Self :: Close) , other => Err (Error :: UnexpectedClientMessage (other)) , } } }
};
}
