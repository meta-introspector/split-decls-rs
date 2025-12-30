// Generated macro for impl_27 (impl)
macro_rules! Depcrate_byte_recordimpl_27 {
() => {
// Module: crate::byte_record
// Provides: {"impl_27"}
// Dependencies: {}
impl < T : AsRef < [u8] > > FromIterator < T > for ByteRecord { # [inline] fn from_iter < I : IntoIterator < Item = T > > (iter : I) -> ByteRecord { let mut record = ByteRecord :: new () ; record . extend (iter) ; record } }
};
}
