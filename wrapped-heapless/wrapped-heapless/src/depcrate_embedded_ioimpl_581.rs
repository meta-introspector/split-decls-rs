// Generated macro for impl_581 (impl)
macro_rules! Depcrate_embedded_ioimpl_581 {
() => {
// Module: crate::embedded_io
// Provides: {"impl_581"}
// Dependencies: {}
impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > Write for VecInner < u8 , LenT , S > { # [inline] fn write (& mut self , buf : & [u8]) -> Result < usize , Self :: Error > { self . extend_from_slice (buf) ? ; Ok (buf . len ()) } # [inline] fn flush (& mut self) -> Result < () , Self :: Error > { Ok (()) } }
};
}
