// Generated macro for impl_ordered_float_pow (macro)
macro_rules! Depcrateimpl_ordered_float_pow {
() => {
// Module: crate
// Provides: {"impl_ordered_float_pow"}
// Dependencies: {}
macro_rules ! impl_ordered_float_pow { ($ inner : ty , $ rhs : ty) => { # [cfg (any (feature = "std" , feature = "libm"))] impl Pow <$ rhs > for OrderedFloat <$ inner > { type Output = OrderedFloat <$ inner >; # [inline] fn pow (self , rhs : $ rhs) -> OrderedFloat <$ inner > { OrderedFloat (<$ inner >:: pow (self . 0 , rhs)) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow <&'a $ rhs > for OrderedFloat <$ inner > { type Output = OrderedFloat <$ inner >; # [inline] fn pow (self , rhs : &'a $ rhs) -> OrderedFloat <$ inner > { OrderedFloat (<$ inner >:: pow (self . 0 , * rhs)) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow <$ rhs > for &'a OrderedFloat <$ inner > { type Output = OrderedFloat <$ inner >; # [inline] fn pow (self , rhs : $ rhs) -> OrderedFloat <$ inner > { OrderedFloat (<$ inner >:: pow (self . 0 , rhs)) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a , 'b > Pow <&'a $ rhs > for &'b OrderedFloat <$ inner > { type Output = OrderedFloat <$ inner >; # [inline] fn pow (self , rhs : &'a $ rhs) -> OrderedFloat <$ inner > { OrderedFloat (<$ inner >:: pow (self . 0 , * rhs)) } } } ; }
};
}
