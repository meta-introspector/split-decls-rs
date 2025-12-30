// Generated macro for kill_process (function)
macro_rules! Depcrate_processkill_process {
() => {
// Module: crate::process
// Provides: {"kill_process"}
// Dependencies: {}
fn kill_process (proc : HANDLE , code : u32) -> Result < () , Error > { unsafe { TerminateProcess (proc , code) ? } ; Ok (()) }
};
}
