// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl Conclusion { # [doc = " Returns an exit code that can be returned from `main` to signal"] # [doc = " success/failure to the calling process."] pub fn exit_code (& self) -> ExitCode { if self . has_failed () { ExitCode :: from (101) } else { ExitCode :: SUCCESS } } # [doc = " Returns whether there have been any failures."] pub fn has_failed (& self) -> bool { self . num_failed > 0 } # [doc = " Exits the application with an appropriate error code (0 if all tests"] # [doc = " have passed, 101 if there have been failures). This uses"] # [doc = " [`process::exit`], meaning that destructors are not ran. Consider"] # [doc = " using [`Self::exit_code`] instead for a proper program cleanup."] pub fn exit (& self) -> ! { self . exit_if_failed () ; process :: exit (0) ; } # [doc = " Exits the application with error code 101 if there were any failures."] # [doc = " Otherwise, returns normally. This uses [`process::exit`], meaning that"] # [doc = " destructors are not ran. Consider using [`Self::exit_code`] instead for"] # [doc = " a proper program cleanup."] pub fn exit_if_failed (& self) { if self . has_failed () { process :: exit (101) } } fn empty () -> Self { Self { num_filtered_out : 0 , num_passed : 0 , num_failed : 0 , num_ignored : 0 , num_measured : 0 , } } }
};
}
