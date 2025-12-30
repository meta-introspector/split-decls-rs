// Generated macro for Flag (enum)
macro_rules! Depcrate_eventFlag {
() => {
// Module: crate::event
// Provides: {"Flag"}
// Dependencies: {}
# [doc = " Special Notify flag on the event."] # [doc = ""] # [doc = " This attribute is used to flag certain kinds of events that Notify either marks or generates in"] # [doc = " particular ways."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [cfg_attr (all (feature = "serde" , not (feature = "serialization-compat-6")) , serde (rename_all = "camelCase"))] pub enum Flag { # [doc = " Rescan notices are emitted by some platforms (and may also be emitted by Notify itself)."] # [doc = " They indicate either a lapse in the events or a change in the filesystem such that events"] # [doc = " received so far can no longer be relied on to represent the state of the filesystem now."] # [doc = ""] # [doc = " An application that simply reacts to file changes may not care about this. An application"] # [doc = " that keeps an in-memory representation of the filesystem will need to care, and will need"] # [doc = " to refresh that representation directly from the filesystem."] Rescan , }
};
}
