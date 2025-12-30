// Generated macro for BufferFormat (enum)
macro_rules! Depcrate_bufBufferFormat {
() => {
// Module: crate::buf
// Provides: {"BufferFormat"}
// Dependencies: {}
# [doc = " An enum expressing all Serde formats known to ICU4X."] # [derive (Debug , PartialEq , Eq , Hash , Copy , Clone)] # [cfg_attr (feature = "serde" , derive (:: serde :: Serialize , :: serde :: Deserialize))] # [non_exhaustive] pub enum BufferFormat { # [doc = " Serialize using JavaScript Object Notation (JSON), using the [`serde_json`] crate."] Json , # [doc = " Serialize using the [`bincode`] crate, version 1."] Bincode1 , # [doc = " Serialize using the [`postcard`] crate, version 1."] Postcard1 , }
};
}
