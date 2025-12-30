// Generated macro for JsonBundleWriteOutcome (struct)
macro_rules! Depcrate_pack_receiveJsonBundleWriteOutcome {
() => {
// Module: crate::pack::receive
// Provides: {"JsonBundleWriteOutcome"}
// Dependencies: {}
# [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct JsonBundleWriteOutcome { pub index_version : pack :: index :: Version , pub index_hash : String , pub data_hash : String , pub num_objects : u32 , }
};
}
