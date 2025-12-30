// Generated macro for impl_184 (impl)
macro_rules! Depcrate_cal_hebrewimpl_184 {
() => {
// Module: crate::cal::hebrew
// Provides: {"impl_184"}
// Dependencies: {}
impl PackWithMD for HebrewYear { # [doc = " The first byte is the [`Keviyah`], the remaining four the YMD as encoded by [`i32::pack`]."] type Packed = [u8 ; 5] ; fn pack (self , month : u8 , day : u8) -> Self :: Packed { let a = self . keviyah as u8 ; let [b , c , d , e] = self . value . pack (month , day) ; [a , b , c , d , e] } fn unpack_year ([a , b , c , d , e] : Self :: Packed) -> Self { let value = i32 :: unpack_year ([b , c , d , e]) ; let keviyah = Keviyah :: from_integer (a) ; Self { keviyah , value } } fn unpack_month ([_ , b , c , d , e] : Self :: Packed) -> u8 { i32 :: unpack_month ([b , c , d , e]) } fn unpack_day ([_ , b , c , d , e] : Self :: Packed) -> u8 { i32 :: unpack_day ([b , c , d , e]) } }
};
}
