// Generated macro for impl_428 (impl)
macro_rules! Depcrate_handlesimpl_428 {
() => {
// Module: crate::handles
// Provides: {"impl_428"}
// Dependencies: {}
impl < 'a , 'b > ByteFourHandle < 'a , 'b > where 'b : 'a , { # [inline (always)] fn new (dst : & 'a mut ByteDestination < 'b >) -> ByteFourHandle < 'a , 'b > { ByteFourHandle { dest : dst } } # [inline (always)] pub fn written (& self) -> usize { self . dest . written () } # [inline (always)] pub fn write_one (self , first : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_one (first) ; self . dest } # [inline (always)] pub fn write_two (self , first : u8 , second : u8) -> & 'a mut ByteDestination < 'b > { self . dest . write_two (first , second) ; self . dest } # [inline (always)] pub fn write_four (self , first : u8 , second : u8 , third : u8 , fourth : u8 ,) -> & 'a mut ByteDestination < 'b > { self . dest . write_four (first , second , third , fourth) ; self . dest } }
};
}
