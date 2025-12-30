// Generated macro for impl_31 (impl)
macro_rules! Depcrate_subscriptionsimpl_31 {
() => {
// Module: crate::subscriptions
// Provides: {"impl_31"}
// Dependencies: {}
impl < S : ScalarValue > TryFrom < Message > for graphql_ws :: ClientMessage < S > { type Error = Error ; fn try_from (msg : Message) -> Result < Self , Self :: Error > { match msg . 0 { ws :: Message :: Text (text) => { serde_json :: from_slice (text . as_bytes ()) . map_err (Error :: Serde) } ws :: Message :: Binary (bytes) => { serde_json :: from_slice (bytes . as_ref ()) . map_err (Error :: Serde) } ws :: Message :: Close (_) => Ok (Self :: ConnectionTerminate) , other => Err (Error :: UnexpectedClientMessage (other)) , } } }
};
}
