// Generated macro for Process (struct)
macro_rules! Depcrate_processProcess {
() => {
// Module: crate::process
// Provides: {"Process"}
// Dependencies: {}
# [doc = " The structure is resposible for interations with spawned process."] # [doc = " It handles IO and other operations related to a spawned process."] pub struct Process { input : HANDLE , output : HANDLE , _proc : PROCESS_INFORMATION , _proc_info : STARTUPINFOEXW , _console : HPCON , }
};
}
