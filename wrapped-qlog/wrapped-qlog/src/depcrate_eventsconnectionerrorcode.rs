// Generated macro for ConnectionErrorCode (enum)
macro_rules! Depcrate_eventsConnectionErrorCode {
() => {
// Module: crate::events
// Provides: {"ConnectionErrorCode"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (untagged)] pub enum ConnectionErrorCode { TransportError (TransportError) , CryptoError (CryptoError) , Value (u64) , }
};
}
