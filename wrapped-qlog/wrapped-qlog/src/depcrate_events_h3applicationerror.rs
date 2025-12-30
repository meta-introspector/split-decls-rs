// Generated macro for ApplicationError (enum)
macro_rules! Depcrate_events_h3ApplicationError {
() => {
// Module: crate::events::h3
// Provides: {"ApplicationError"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum ApplicationError { HttpNoError , HttpGeneralProtocolError , HttpInternalError , HttpRequestCancelled , HttpIncompleteRequest , HttpConnectError , HttpFrameError , HttpExcessiveLoad , HttpVersionFallback , HttpIdError , HttpStreamCreationError , HttpClosedCriticalStream , HttpEarlyResponse , HttpMissingSettings , HttpUnexpectedFrame , HttpRequestRejection , HttpSettingsError , Unknown , }
};
}
