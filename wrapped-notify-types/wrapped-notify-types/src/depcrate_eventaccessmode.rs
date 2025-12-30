// Generated macro for AccessMode (enum)
macro_rules! Depcrate_eventAccessMode {
() => {
// Module: crate::event
// Provides: {"AccessMode"}
// Dependencies: {}
# [doc = " An event describing open or close operations on files."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum AccessMode { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted when the file is executed, or the folder opened."] Execute , # [doc = " An event emitted when the file is opened for reading."] Read , # [doc = " An event emitted when the file is opened for writing."] Write , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
