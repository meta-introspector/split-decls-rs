// Generated macro for impl_573 (impl)
macro_rules! Depcrate_ufmtimpl_573 {
() => {
// Module: crate::ufmt
// Provides: {"impl_573"}
// Dependencies: {}
impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > uWrite for VecInner < u8 , LenT , S > { type Error = CapacityError ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . extend_from_slice (s . as_bytes ()) } }
};
}
