// Generated macro for impl_574 (impl)
macro_rules! Depcrate_ufmtimpl_574 {
() => {
// Module: crate::ufmt
// Provides: {"impl_574"}
// Dependencies: {}
impl < const N : usize , LenT : LenType > uWrite for CString < N , LenT > { type Error = c_string :: ExtendError ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . extend_from_bytes (s . as_bytes ()) } }
};
}
