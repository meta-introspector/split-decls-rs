// Generated macro for impl_424 (impl)
macro_rules! Depcrate_handlesimpl_424 {
() => {
// Module: crate::handles
// Provides: {"impl_424"}
// Dependencies: {}
impl < 'a , 'b > ByteTwoHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut ByteDestination < 'b >) -> ByteTwoHandle < 'a , 'b > { ByteTwoHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_one (self , first : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_one (first) ; self . dest } # [inline (always)] pub fn write_two (self , first : u8 , second : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_two (first , second) ; self . dest } }
};
}
