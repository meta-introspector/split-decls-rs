// Generated macro for MtuUpdated (struct)
macro_rules! Depcrate_events_connectivityMtuUpdated {
() => {
// Module: crate::events::connectivity
// Provides: {"MtuUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct MtuUpdated { pub old : Option < u16 > , pub new : u16 , pub done : Option < bool > , }
};
}
