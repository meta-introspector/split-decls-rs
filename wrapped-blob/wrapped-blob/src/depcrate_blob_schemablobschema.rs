// Generated macro for BlobSchema (enum)
macro_rules! Depcrate_blob_schemaBlobSchema {
() => {
// Module: crate::blob_schema
// Provides: {"BlobSchema"}
// Dependencies: {}
# [doc = " A versioned Serde schema for ICU4X data blobs."] # [derive (serde :: Deserialize , yoke :: Yokeable)] # [yoke (prove_covariance_manually)] # [cfg_attr (feature = "export" , derive (serde :: Serialize))] # [derive (Debug , Clone)] pub (crate) enum BlobSchema < 'data > { V001 (NeverSchema) , V002 (NeverSchema) , V002Bigger (NeverSchema) , # [serde (borrow)] V003 (BlobSchemaV1 < 'data , Index16 >) , # [serde (borrow)] V003Bigger (BlobSchemaV1 < 'data , Index32 >) , }
};
}
