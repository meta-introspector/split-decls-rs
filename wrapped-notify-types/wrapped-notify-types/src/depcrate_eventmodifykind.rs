// Generated macro for ModifyKind (enum)
macro_rules! Depcrate_eventModifyKind {
() => {
// Module: crate::event
// Provides: {"ModifyKind"}
// Dependencies: {}
# [doc = " An event describing mutation of content, name, or metadata."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (tag = "kind" , content = "mode"))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum ModifyKind { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted when the data content of a file is changed."] Data (DataChange) , # [doc = " An event emitted when the metadata of a file or folder is changed."] Metadata (MetadataKind) , # [doc = " An event emitted when the name of a file or folder is changed."] # [cfg_attr (feature = "serde" , serde (rename = "rename"))] Name (RenameMode) , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
