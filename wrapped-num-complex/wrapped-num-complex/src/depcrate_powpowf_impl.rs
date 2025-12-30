// Generated macro for powf_impl (macro)
macro_rules! Depcrate_powpowf_impl {
() => {
// Module: crate::pow
// Provides: {"powf_impl"}
// Dependencies: {}
macro_rules ! powf_impl { ($ F : ty) => { # [cfg (any (feature = "std" , feature = "libm"))] impl <'a , T : Float > Pow <$ F > for &'a Complex < T > where $ F : Into < T >, { type Output = Complex < T >; # [inline] fn pow (self , exp : $ F) -> Self :: Output { self . powf (exp . into ()) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a , 'b , T : Float > Pow <&'b $ F > for &'a Complex < T > where $ F : Into < T >, { type Output = Complex < T >; # [inline] fn pow (self , & exp : &$ F) -> Self :: Output { self . powf (exp . into ()) } } # [cfg (any (feature = "std" , feature = "libm"))] impl < T : Float > Pow <$ F > for Complex < T > where $ F : Into < T >, { type Output = Complex < T >; # [inline] fn pow (self , exp : $ F) -> Self :: Output { self . powf (exp . into ()) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'b , T : Float > Pow <&'b $ F > for Complex < T > where $ F : Into < T >, { type Output = Complex < T >; # [inline] fn pow (self , & exp : &$ F) -> Self :: Output { self . powf (exp . into ()) } } } ; }
};
}
