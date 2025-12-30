// Generated macro for AccessKind (enum)
macro_rules! Depcrate_eventAccessKind {
() => {
// Module: crate::event
// Provides: {"AccessKind"}
// Dependencies: {}
# [doc = " An event describing non-mutating access operations on files."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (tag = "kind" , content = "mode"))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum AccessKind { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted when the file is read."] Read , # [doc = " An event emitted when the file, or a handle to the file, is opened."] Open (AccessMode) , # [doc = " An event emitted when the file, or a handle to the file, is closed."] Close (AccessMode) , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
