// Generated macro for JsonOutcome (struct)
macro_rules! Depcrate_pack_receiveJsonOutcome {
() => {
// Module: crate::pack::receive
// Provides: {"JsonOutcome"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct JsonOutcome { pub index : JsonBundleWriteOutcome , pub pack_kind : pack :: data :: Version , pub index_path : Option < PathBuf > , pub data_path : Option < PathBuf > , pub refs : Vec < crate :: repository :: remote :: JsonRef > , }
};
}
