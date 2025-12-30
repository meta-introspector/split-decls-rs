// Generated macro for impl_349 (impl)
macro_rules! Depcrate_vecimpl_349 {
() => {
// Module: crate::vec
// Provides: {"impl_349"}
// Dependencies: {}
impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > fmt :: Write for VecInner < u8 , LenT , S > { fn write_str (& mut self , s : & str) -> fmt :: Result { match self . extend_from_slice (s . as_bytes ()) { Ok (()) => Ok (()) , Err (_) => Err (fmt :: Error) , } } }
};
}
