// Generated macro for impl_18 (impl)
macro_rules! Depcrate_decodeimpl_18 {
() => {
// Module: crate::decode
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'src > Decode < 'src > for u32 { fn decode (data : & mut & 'src [u8]) -> Self { let mut cur = 0 ; let mut offset = 0 ; loop { let byte = get (data) ; cur |= ((byte & 0x7f) as u32) << offset ; if byte & 0x80 == 0 { break cur ; } offset += 7 ; } } }
};
}
