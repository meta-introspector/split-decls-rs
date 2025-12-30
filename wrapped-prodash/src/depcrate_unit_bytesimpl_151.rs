// Generated macro for impl_151 (impl)
macro_rules! Depcrate_unit_bytesimpl_151 {
() => {
// Module: crate::unit::bytes
// Provides: {"impl_151"}
// Dependencies: {}
impl Bytes { fn format_bytes (w : & mut dyn fmt :: Write , value : Step) -> fmt :: Result { let string = bytesize :: ByteSize (value as u64) . display () . si () . to_string () ; for token in string . split (' ') { w . write_str (token) ? ; } Ok (()) } }
};
}
