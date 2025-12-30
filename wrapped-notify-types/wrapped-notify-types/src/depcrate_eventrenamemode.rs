// Generated macro for RenameMode (enum)
macro_rules! Depcrate_eventRenameMode {
() => {
// Module: crate::event
// Provides: {"RenameMode"}
// Dependencies: {}
# [doc = " An event emitted when the name of a file or folder is changed."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum RenameMode { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted on the file or folder resulting from a rename."] To , # [doc = " An event emitted on the file or folder that was renamed."] From , # [doc = " A single event emitted with both the `From` and `To` paths."] # [doc = ""] # [doc = " This event should be emitted when both source and target are known. The paths should be"] # [doc = " provided in this exact order (from, to)."] Both , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
