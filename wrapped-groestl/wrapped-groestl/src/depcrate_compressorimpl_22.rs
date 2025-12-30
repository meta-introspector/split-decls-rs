// Generated macro for impl_22 (impl)
macro_rules! Depcrate_compressorimpl_22 {
() => {
// Module: crate::compressor
// Provides: {"impl_22"}
// Dependencies: {}
impl X8 { # [inline (always)] fn map < F > (self , mut f : F) -> Self where F : FnMut (__m128i) -> __m128i , { X8 (f (self . 0) , f (self . 1) , f (self . 2) , f (self . 3) , f (self . 4) , f (self . 5) , f (self . 6) , f (self . 7) ,) } # [inline (always)] fn shuffle (self , i : (usize , usize , usize , usize , usize , usize , usize , usize)) -> Self { let xs = [self . 0 , self . 1 , self . 2 , self . 3 , self . 4 , self . 5 , self . 6 , self . 7 ,] ; X8 (xs [i . 0] , xs [i . 1] , xs [i . 2] , xs [i . 3] , xs [i . 4] , xs [i . 5] , xs [i . 6] , xs [i . 7] ,) } # [inline (always)] fn rotl1 (self) -> Self { self . shuffle ((1 , 2 , 3 , 4 , 5 , 6 , 7 , 0)) } # [inline (always)] fn rotl2 (self) -> Self { self . shuffle ((2 , 3 , 4 , 5 , 6 , 7 , 0 , 1)) } # [inline (always)] fn rotl3 (self) -> Self { self . shuffle ((3 , 4 , 5 , 6 , 7 , 0 , 1 , 2)) } # [inline (always)] fn rotl4 (self) -> Self { self . shuffle ((4 , 5 , 6 , 7 , 0 , 1 , 2 , 3)) } # [inline (always)] fn rotl6 (self) -> Self { self . shuffle ((6 , 7 , 0 , 1 , 2 , 3 , 4 , 5)) } }
};
}
