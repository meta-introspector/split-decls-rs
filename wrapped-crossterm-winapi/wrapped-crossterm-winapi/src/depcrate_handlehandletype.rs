// Generated macro for HandleType (enum)
macro_rules! Depcrate_handleHandleType {
() => {
// Module: crate::handle
// Provides: {"HandleType"}
// Dependencies: {}
# [doc = " The standard handles of a process."] # [doc = ""] # [doc = " See [the Windows documentation on console"] # [doc = " handles](https://docs.microsoft.com/en-us/windows/console/console-handles) for more info."] # [derive (Debug , Clone , Copy)] pub enum HandleType { # [doc = " The process' standard output."] OutputHandle , # [doc = " The process' standard input."] InputHandle , # [doc = " The process' active console screen buffer, `CONOUT$`."] CurrentOutputHandle , # [doc = " The process' console input buffer, `CONIN$`."] CurrentInputHandle , }
};
}
