// Generated macro for impl_572 (impl)
macro_rules! Depcrate_ufmtimpl_572 {
() => {
// Module: crate::ufmt
// Provides: {"impl_572"}
// Dependencies: {}
impl < LenT : LenType , S : StringStorage + ? Sized > uWrite for StringInner < LenT , S > { type Error = CapacityError ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . push_str (s) } }
};
}
