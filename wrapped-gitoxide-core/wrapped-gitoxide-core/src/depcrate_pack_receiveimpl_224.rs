// Generated macro for impl_224 (impl)
macro_rules! Depcrate_pack_receiveimpl_224 {
() => {
// Module: crate::pack::receive
// Provides: {"impl_224"}
// Dependencies: {}
impl From < pack :: index :: write :: Outcome > for JsonBundleWriteOutcome { fn from (v : pack :: index :: write :: Outcome) -> Self { JsonBundleWriteOutcome { index_version : v . index_version , num_objects : v . num_objects , data_hash : v . data_hash . to_string () , index_hash : v . index_hash . to_string () , } } }
};
}
