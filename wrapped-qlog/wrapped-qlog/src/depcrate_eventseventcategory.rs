// Generated macro for EventCategory (enum)
macro_rules! Depcrate_eventsEventCategory {
() => {
// Module: crate::events
// Provides: {"EventCategory"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Debug)] # [serde (rename_all = "snake_case")] pub enum EventCategory { Connectivity , Security , Transport , Recovery , Http , Qpack , Error , Warning , Info , Debug , Verbose , Simulation , }
};
}
