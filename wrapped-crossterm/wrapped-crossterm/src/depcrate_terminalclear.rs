// Generated macro for Clear (struct)
macro_rules! Depcrate_terminalClear {
() => {
// Module: crate::terminal
// Provides: {"Clear"}
// Dependencies: {}
# [doc = " A command that clears the terminal screen buffer."] # [doc = ""] # [doc = " See the [`ClearType`](enum.ClearType.html) enum."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct Clear (pub ClearType) ;
};
}
