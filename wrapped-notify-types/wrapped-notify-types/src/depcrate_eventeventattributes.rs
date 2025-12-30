// Generated macro for EventAttributes (struct)
macro_rules! Depcrate_eventEventAttributes {
() => {
// Module: crate::event
// Provides: {"EventAttributes"}
// Dependencies: {}
# [doc = " Additional attributes of the event."] # [derive (Clone , Default , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct EventAttributes { # [cfg_attr (feature = "serde" , serde (flatten))] inner : Option < Box < EventAttributesInner > > , }
};
}
