// Generated macro for EventType (enum)
macro_rules! Depcrate_eventsEventType {
() => {
// Module: crate::events
// Provides: {"EventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug , Default)] # [serde (untagged)] pub enum EventType { ConnectivityEventType (ConnectivityEventType) , TransportEventType (TransportEventType) , SecurityEventType (SecurityEventType) , RecoveryEventType (RecoveryEventType) , Http3EventType (Http3EventType) , QpackEventType (QpackEventType) , GenericEventType (GenericEventType) , # [default] None , }
};
}
