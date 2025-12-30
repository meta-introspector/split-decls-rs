// Generated macro for CreateKind (enum)
macro_rules! Depcrate_eventCreateKind {
() => {
// Module: crate::event
// Provides: {"CreateKind"}
// Dependencies: {}
# [doc = " An event describing creation operations on files."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (tag = "kind"))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum CreateKind { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event which results in the creation of a file."] File , # [doc = " An event which results in the creation of a folder."] Folder , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
