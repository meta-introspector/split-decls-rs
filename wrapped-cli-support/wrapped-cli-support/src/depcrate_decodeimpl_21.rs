// Generated macro for impl_21 (impl)
macro_rules! Depcrate_decodeimpl_21 {
() => {
// Module: crate::decode
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'src , T : Decode < 'src > > Decode < 'src > for Vec < T > { fn decode (data : & mut & 'src [u8]) -> Self { let n = u32 :: decode (data) ; let mut v = Vec :: with_capacity (n as usize) ; log :: trace ! ("found a list of length {n}") ; for _ in 0 .. n { v . push (Decode :: decode (data)) ; } v } }
};
}
