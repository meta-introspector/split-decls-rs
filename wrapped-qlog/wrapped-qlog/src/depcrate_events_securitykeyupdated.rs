// Generated macro for KeyUpdated (struct)
macro_rules! Depcrate_events_securityKeyUpdated {
() => {
// Module: crate::events::security
// Provides: {"KeyUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug , Default)] pub struct KeyUpdated { pub key_type : KeyType , pub old : Option < Bytes > , pub new : Bytes , pub generation : Option < u32 > , pub trigger : Option < KeyUpdateOrRetiredTrigger > , }
};
}
