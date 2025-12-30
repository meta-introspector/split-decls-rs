// Generated macro for use_2574 (pub_use)
macro_rules! Depcrate_lockuse_2574 {
() => {
// Module: crate::lock
// Provides: {"use_2574"}
// Dependencies: {}
# [cfg_attr (target_os = "none" , cfg (target_has_atomic = "ptr"))] # [cfg (feature = "bilock")] # [cfg_attr (docsrs , doc (cfg (feature = "bilock")))] pub use self :: bilock :: { BiLock , BiLockAcquire , BiLockGuard , ReuniteError } ;
};
}
