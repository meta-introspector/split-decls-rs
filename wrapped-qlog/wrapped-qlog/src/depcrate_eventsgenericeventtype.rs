// Generated macro for GenericEventType (enum)
macro_rules! Depcrate_eventsGenericEventType {
() => {
// Module: crate::events
// Provides: {"GenericEventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum GenericEventType { ConnectionError , ApplicationError , InternalError , InternalWarning , Message , Marker , }
};
}
