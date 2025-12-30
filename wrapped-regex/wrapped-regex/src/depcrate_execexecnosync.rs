// Generated macro for ExecNoSync (struct)
macro_rules! Depcrate_execExecNoSync {
() => {
// Module: crate::exec
// Provides: {"ExecNoSync"}
// Dependencies: {}
# [doc = " ExecNoSync is like Exec, except it embeds a reference to a cache. This"] # [doc = " means it is no longer Sync, but we can now avoid the overhead of"] # [doc = " synchronization to fetch the cache."] # [derive (Debug)] pub struct ExecNoSync < 'c > { # [doc = " All read only state."] ro : & 'c Arc < ExecReadOnly > , # [doc = " Caches for the various matching engines."] cache : & 'c ProgramCache , }
};
}
