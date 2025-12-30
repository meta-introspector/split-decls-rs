// Generated macro for ProcMacroClient (struct)
macro_rules! DepcrateProcMacroClient {
() => {
// Module: crate
// Provides: {"ProcMacroClient"}
// Dependencies: {}
# [doc = " A handle to an external process which load dylibs with macros (.so or .dll)"] # [doc = " and runs actual macro expansion functions."] # [derive (Debug)] pub struct ProcMacroClient { # [doc = " Currently, the proc macro process expands all procedural macros sequentially."] # [doc = ""] # [doc = " That means that concurrent salsa requests may block each other when expanding proc macros,"] # [doc = " which is unfortunate, but simple and good enough for the time being."] process : Arc < ProcMacroServerProcess > , path : AbsPathBuf , }
};
}
