// Generated macro for Time (struct)
macro_rules! Depcrate_entry_statTime {
() => {
// Module: crate::entry::stat
// Provides: {"Time"}
// Dependencies: {}
# [doc = " The time component in a [`Stat`] struct."] # [derive (Debug , Default , PartialEq , Eq , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct Time { # [doc = " The amount of seconds elapsed since EPOCH."] pub secs : u32 , # [doc = " The amount of nanoseconds elapsed in the current second, ranging from 0 to 999.999.999 ."] pub nsecs : u32 , }
};
}
