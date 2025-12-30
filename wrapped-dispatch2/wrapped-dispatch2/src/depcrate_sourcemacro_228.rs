// Generated macro for macro_228 (macro)
macro_rules! Depcrate_sourcemacro_228 {
() => {
// Module: crate::source
// Provides: {"macro_228"}
// Dependencies: {}
enum_with_val ! { # [doc = " Events related to a process."] # [derive (PartialEq , Eq , Clone , Copy)] pub struct dispatch_source_proc_flags_t (pub c_ulong) { DISPATCH_PROC_EXIT = 0x80000000 , DISPATCH_PROC_FORK = 0x40000000 , DISPATCH_PROC_EXEC = 0x20000000 , DISPATCH_PROC_SIGNAL = 0x08000000 , } }
};
}
