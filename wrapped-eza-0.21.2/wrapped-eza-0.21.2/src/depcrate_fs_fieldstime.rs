// Generated macro for Time (struct)
macro_rules! Depcrate_fs_fieldsTime {
() => {
// Module: crate::fs::fields
// Provides: {"Time"}
// Dependencies: {}
# [doc = " One of a file’s timestamps (created, accessed, or modified)."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct Time { pub seconds : time_t , pub nanoseconds : time_t , }
};
}
