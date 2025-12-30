// Generated macro for ProcessError (struct)
macro_rules! Depcrate_process_errorProcessError {
() => {
// Module: crate::process_error
// Provides: {"ProcessError"}
// Dependencies: {}
# [derive (Debug)] pub struct ProcessError { # [doc = " A detailed description to show to the user why the process failed."] pub desc : String , # [doc = " The exit status of the process."] # [doc = ""] # [doc = " This can be `None` if the process failed to launch (like process not"] # [doc = " found) or if the exit status wasn't a code but was instead something"] # [doc = " like termination via a signal."] pub code : Option < i32 > , # [doc = " The stdout from the process."] # [doc = ""] # [doc = " This can be `None` if the process failed to launch, or the output was"] # [doc = " not captured."] pub stdout : Option < Vec < u8 > > , # [doc = " The stderr from the process."] # [doc = ""] # [doc = " This can be `None` if the process failed to launch, or the output was"] # [doc = " not captured."] pub stderr : Option < Vec < u8 > > , }
};
}
