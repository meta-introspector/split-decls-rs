// Generated macro for Event (struct)
macro_rules! Depcrate_eventEvent {
() => {
// Module: crate::event
// Provides: {"Event"}
// Dependencies: {}
# [derive (Clone , Eq , PartialEq , Hash , Debug)] pub struct Event < 'a > { pub event_kind : Cow < 'a , str > , pub label : Cow < 'a , str > , pub additional_data : Vec < Cow < 'a , str > > , pub payload : EventPayload , pub thread_id : u32 , }
};
}
