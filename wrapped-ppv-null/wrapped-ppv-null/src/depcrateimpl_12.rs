// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl u32x4x4 { # [inline (always)] fn zipmap < F > (self , rhs : Self , mut f : F) -> Self where F : FnMut (u32x4 , u32x4) -> u32x4 , { u32x4x4 (f (self . 0 , rhs . 0) , f (self . 1 , rhs . 1) , f (self . 2 , rhs . 2) , f (self . 3 , rhs . 3) ,) } # [inline (always)] pub fn from ((a , b , c , d) : (u32x4 , u32x4 , u32x4 , u32x4)) -> Self { u32x4x4 (a , b , c , d) } # [inline (always)] pub fn splat (a : u32x4) -> Self { u32x4x4 (a , a , a , a) } # [inline (always)] pub fn into_parts (self) -> (u32x4 , u32x4 , u32x4 , u32x4) { (self . 0 , self . 1 , self . 2 , self . 3) } }
};
}
