// Generated macro for Mode (enum)
macro_rules! Depcrate_driver_shutdownMode {
() => {
// Module: crate::driver::shutdown
// Provides: {"Mode"}
// Dependencies: {}
# [doc = ""] # [derive (Debug , Copy , Clone)] pub enum Mode { # [doc = " Wait for long-running processes after signaling them to shut down by closing their input and output."] WaitForProcesses , # [doc = " Do not do anything with long-running processes, which typically allows them to keep running or shut down on their own time."] # [doc = " This is the fastest mode as no synchronization happens at all."] Ignore , }
};
}
