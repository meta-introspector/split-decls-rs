// Generated macro for impl_422 (impl)
macro_rules! Depcrate_handlesimpl_422 {
() => {
// Module: crate::handles
// Provides: {"impl_422"}
// Dependencies: {}
impl < 'a , 'b > ByteOneHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut ByteDestination < 'b >) -> ByteOneHandle < 'a , 'b > { ByteOneHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_one (self , first : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_one (first) ; self . dest } }
};
}
