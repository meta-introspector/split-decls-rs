// Generated macro for TermFamily (enum)
macro_rules! Depcrate_termTermFamily {
() => {
// Module: crate::term
// Provides: {"TermFamily"}
// Dependencies: {}
# [doc = " The family of the terminal."] # [derive (Debug , Copy , Clone , PartialEq , Eq)] pub enum TermFamily { # [doc = " Redirected to a file or file like thing."] File , # [doc = " A standard unix terminal."] UnixTerm , # [doc = " A cmd.exe like windows console."] WindowsConsole , # [doc = " A dummy terminal (for instance on wasm)"] Dummy , }
};
}
