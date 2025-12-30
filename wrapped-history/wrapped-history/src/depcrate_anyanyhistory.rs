// Generated macro for AnyHistory (enum)
macro_rules! Depcrate_anyAnyHistory {
() => {
// Module: crate::any
// Provides: {"AnyHistory"}
// Dependencies: {}
# [doc = " A [`History`] that provides a universal API to the underlying history type."] # [derive (Clone , PartialEq , Debug)] pub enum AnyHistory { # [doc = " A Browser History."] Browser (BrowserHistory) , # [doc = " A Hash History"] Hash (HashHistory) , # [doc = " A Memory History"] Memory (MemoryHistory) , }
};
}
