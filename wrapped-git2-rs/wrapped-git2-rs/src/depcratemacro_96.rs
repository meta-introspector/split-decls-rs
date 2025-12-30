// Generated macro for macro_96 (macro)
macro_rules! Depcratemacro_96 {
() => {
// Module: crate
// Provides: {"macro_96"}
// Dependencies: {}
bitflags ! { # [doc = " How to handle reference updates."] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct RemoteUpdateFlags : u32 { # [doc = " Write the fetch results to FETCH_HEAD."] const UPDATE_FETCHHEAD = raw :: GIT_REMOTE_UPDATE_FETCHHEAD as u32 ; # [doc = " Report unchanged tips in the update_tips callback."] const REPORT_UNCHANGED = raw :: GIT_REMOTE_UPDATE_REPORT_UNCHANGED as u32 ; } }
};
}
