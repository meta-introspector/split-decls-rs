// Generated macro for ProcMacroServerProcess (struct)
macro_rules! Depcrate_processProcMacroServerProcess {
() => {
// Module: crate::process
// Provides: {"ProcMacroServerProcess"}
// Dependencies: {}
# [doc = " Represents a process handling proc-macro communication."] # [derive (Debug)] pub (crate) struct ProcMacroServerProcess { # [doc = " The state of the proc-macro server process, the protocol is currently strictly sequential"] # [doc = " hence the lock on the state."] state : Mutex < ProcessSrvState > , version : u32 , protocol : Protocol , # [doc = " Populated when the server exits."] exited : OnceLock < AssertUnwindSafe < ServerError > > , }
};
}
