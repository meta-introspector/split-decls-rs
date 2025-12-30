// Generated macro for KeyDiscarded (struct)
macro_rules! Depcrate_events_securityKeyDiscarded {
() => {
// Module: crate::events::security
// Provides: {"KeyDiscarded"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] pub struct KeyDiscarded { pub key_type : KeyType , pub key : Option < Bytes > , pub generation : Option < u32 > , pub trigger : Option < KeyUpdateOrRetiredTrigger > , }
};
}
