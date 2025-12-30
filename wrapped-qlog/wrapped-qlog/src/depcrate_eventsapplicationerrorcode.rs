// Generated macro for ApplicationErrorCode (enum)
macro_rules! Depcrate_eventsApplicationErrorCode {
() => {
// Module: crate::events
// Provides: {"ApplicationErrorCode"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (untagged)] pub enum ApplicationErrorCode { ApplicationError (ApplicationError) , Value (u64) , }
};
}
