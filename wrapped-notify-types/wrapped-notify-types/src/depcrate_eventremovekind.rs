// Generated macro for RemoveKind (enum)
macro_rules! Depcrate_eventRemoveKind {
() => {
// Module: crate::event
// Provides: {"RemoveKind"}
// Dependencies: {}
# [doc = " An event describing removal operations on files."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (tag = "kind"))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum RemoveKind { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted when a file is removed."] File , # [doc = " An event emitted when a folder is removed."] Folder , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
