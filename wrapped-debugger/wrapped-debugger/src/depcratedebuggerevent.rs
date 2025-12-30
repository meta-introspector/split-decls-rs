// Generated macro for DebuggerEvent (enum)
macro_rules! DepcrateDebuggerEvent {
() => {
// Module: crate
// Provides: {"DebuggerEvent"}
// Dependencies: {}
# [doc = " Events that are sent from the debugger."] # [derive (Debug , PartialEq , Eq)] pub enum DebuggerEvent { # [doc = " A breakpoint encountered."] # [doc = " The first element is the rule name."] # [doc = " The second element is the position."] Breakpoint (String , usize) , # [doc = " The end of the input has been reached."] Eof , # [doc = " A parsing error encountered."] Error (String) , }
};
}
