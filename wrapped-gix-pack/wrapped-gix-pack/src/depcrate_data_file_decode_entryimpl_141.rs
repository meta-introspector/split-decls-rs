// Generated macro for impl_141 (impl)
macro_rules! Depcrate_data_file_decode_entryimpl_141 {
() => {
// Module: crate::data::file::decode::entry
// Provides: {"impl_141"}
// Dependencies: {}
impl Outcome { pub (crate) fn default_from_kind (kind : gix_object :: Kind) -> Self { Self { kind , num_deltas : 0 , decompressed_size : 0 , compressed_size : 0 , object_size : 0 , } } fn from_object_entry (kind : gix_object :: Kind , entry : & data :: Entry , compressed_size : usize) -> Self { Self { kind , num_deltas : 0 , decompressed_size : entry . decompressed_size , compressed_size , object_size : entry . decompressed_size , } } }
};
}
