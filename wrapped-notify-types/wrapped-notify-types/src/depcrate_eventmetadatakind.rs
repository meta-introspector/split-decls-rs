// Generated macro for MetadataKind (enum)
macro_rules! Depcrate_eventMetadataKind {
() => {
// Module: crate::event
// Provides: {"MetadataKind"}
// Dependencies: {}
# [doc = " An event emitted when the metadata of a file or folder is changed."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum MetadataKind { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted when the access time of the file or folder is changed."] AccessTime , # [doc = " An event emitted when the write or modify time of the file or folder is changed."] WriteTime , # [doc = " An event emitted when the permissions of the file or folder are changed."] Permissions , # [doc = " An event emitted when the ownership of the file or folder is changed."] Ownership , # [doc = " An event emitted when an extended attribute of the file or folder is changed."] # [doc = ""] # [doc = " If the extended attribute's name or type is known, it should be provided in the"] # [doc = " `Info` event attribute."] Extended , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
