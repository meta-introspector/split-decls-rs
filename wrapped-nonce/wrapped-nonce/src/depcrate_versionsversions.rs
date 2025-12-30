// Generated macro for Versions (enum)
macro_rules! Depcrate_versionsVersions {
() => {
// Module: crate::versions
// Provides: {"Versions"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Debug , PartialEq , Eq , Clone)] pub enum Versions { Legacy (Box < State >) , # [doc = " Current variants have durable nonce and blockhash domains separated."] Current (Box < State >) , }
};
}
