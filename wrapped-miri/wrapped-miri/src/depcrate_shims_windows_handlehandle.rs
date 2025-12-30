// Generated macro for Handle (enum)
macro_rules! Depcrate_shims_windows_handleHandle {
() => {
// Module: crate::shims::windows::handle
// Provides: {"Handle"}
// Dependencies: {}
# [doc = " Miri representation of a Windows `HANDLE`"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum Handle { Null , Pseudo (PseudoHandle) , Thread (ThreadId) , File (FdNum) , Invalid , }
};
}
