// Generated macro for ExitStatus (struct)
macro_rules! Depcrate_concurrency_genmcExitStatus {
() => {
// Module: crate::concurrency::genmc
// Provides: {"ExitStatus"}
// Dependencies: {}
# [doc = " The exit status of a program."] # [doc = " GenMC must store this if a thread exits while any others can still run."] # [doc = " The other threads must also be explored before the program is terminated."] # [derive (Clone , Copy , Debug)] struct ExitStatus { exit_code : i32 , exit_type : ExitType , }
};
}
