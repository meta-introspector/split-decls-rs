// Generated macro for impl_19 (impl)
macro_rules! Depcrate_decodeimpl_19 {
() => {
// Module: crate::decode
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'src > Decode < 'src > for & 'src str { fn decode (data : & mut & 'src [u8]) -> & 'src str { let n = u32 :: decode (data) ; let (a , b) = data . split_at (n as usize) ; * data = b ; let r = str :: from_utf8 (a) . unwrap () ; log :: trace ! ("decoded string {r:?}") ; r } }
};
}
