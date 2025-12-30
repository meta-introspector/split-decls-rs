// Generated macro for RunFailMode (enum)
macro_rules! Depcrate_commonRunFailMode {
() => {
// Module: crate::common
// Provides: {"RunFailMode"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , PartialOrd)] pub enum RunFailMode { # [doc = " Running the program must make it exit with a regular failure exit code"] # [doc = " in the range `1..=127`. If the program is terminated by e.g. a signal"] # [doc = " the test will fail."] Fail , # [doc = " Running the program must result in a crash, e.g. by `SIGABRT` or"] # [doc = " `SIGSEGV` on Unix or on Windows by having an appropriate NTSTATUS high"] # [doc = " bit in the exit code."] Crash , # [doc = " Running the program must either fail or crash. Useful for e.g. sanitizer"] # [doc = " tests since some sanitizer implementations exit the process with code 1"] # [doc = " to in the face of memory errors while others abort (crash) the process"] # [doc = " in the face of memory errors."] FailOrCrash , }
};
}
