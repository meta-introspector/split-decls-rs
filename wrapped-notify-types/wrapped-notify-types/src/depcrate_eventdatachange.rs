// Generated macro for DataChange (enum)
macro_rules! Depcrate_eventDataChange {
() => {
// Module: crate::event
// Provides: {"DataChange"}
// Dependencies: {}
# [doc = " An event emitted when the data content of a file is changed."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde" , serde (rename_all = "kebab-case"))] pub enum DataChange { # [doc = " The catch-all case, to be used when the specific kind of event is unknown."] Any , # [doc = " An event emitted when the size of the data is changed."] Size , # [doc = " An event emitted when the content of the data is changed."] Content , # [doc = " An event which specific kind is known but cannot be represented otherwise."] Other , }
};
}
