// Generated macro for Store (struct)
macro_rules! Depcrate_rt_objectStore {
() => {
// Module: crate::rt::object
// Provides: {"Store"}
// Dependencies: {}
# [doc = " Stores objects"] # [derive (Debug)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (super) struct Store < T = Entry > { # [doc = " Stored state for all objects."] entries : Vec < T > , }
};
}
