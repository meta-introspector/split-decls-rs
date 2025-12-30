// Generated macro for impl_30 (impl)
macro_rules! Depcrate_byte_recordimpl_30 {
() => {
// Module: crate::byte_record
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'r > IntoIterator for & 'r ByteRecord { type IntoIter = ByteRecordIter < 'r > ; type Item = & 'r [u8] ; # [inline] fn into_iter (self) -> ByteRecordIter < 'r > { ByteRecordIter { r : self , last_start : self . as_slice () . len () , last_end : 0 , i_forward : 0 , i_reverse : self . len () , } } }
};
}
