// Generated macro for BlobSchemaV1 (struct)
macro_rules! Depcrate_blob_schemaBlobSchemaV1 {
() => {
// Module: crate::blob_schema
// Provides: {"BlobSchemaV1"}
// Dependencies: {}
# [doc = " Version 3 of the ICU4X data blob schema."] # [doc = ""] # [doc = " This itself has two modes, using [`Index16`] or [`Index32`] buffers for the locales array."] # [doc = ""] # [doc = " The exporter will autoupgrade to the larger buffer as needed."] # [derive (Clone , Copy , Debug , serde :: Deserialize , yoke :: Yokeable)] # [yoke (prove_covariance_manually)] # [cfg_attr (feature = "export" , derive (serde :: Serialize))] # [serde (bound = "")] pub (crate) struct BlobSchemaV1 < 'data , LocaleVecFormat : VarZeroVecFormat > { # [doc = " Map from marker hash to locale trie."] # [doc = " Weak invariant: should be sorted."] # [serde (borrow)] pub markers : & 'data ZeroSlice < DataMarkerIdHash > , # [doc = " Map from locale to buffer index."] # [doc = " Weak invariant: the `usize` values are valid indices into `self.buffers`"] # [doc = " Weak invariant: there is at least one value for every integer in 0..self.buffers.len()"] # [doc = " Weak invariant: markers and locales are the same length"] # [serde (borrow)] pub locales : & 'data VarZeroSlice < [u8] , LocaleVecFormat > , # [doc = " Vector of buffers"] # [serde (borrow)] pub buffers : & 'data VarZeroSlice < [u8] , Index32 > , }
};
}
