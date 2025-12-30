// Generated macro for impl_ordered_float_self_pow (macro)
macro_rules! Depcrateimpl_ordered_float_self_pow {
() => {
// Module: crate
// Provides: {"impl_ordered_float_self_pow"}
// Dependencies: {}
macro_rules ! impl_ordered_float_self_pow { ($ base : ty , $ exp : ty) => { # [cfg (any (feature = "std" , feature = "libm"))] impl Pow < OrderedFloat <$ exp >> for OrderedFloat <$ base > { type Output = OrderedFloat <$ base >; # [inline] fn pow (self , rhs : OrderedFloat <$ exp >) -> OrderedFloat <$ base > { OrderedFloat (<$ base >:: pow (self . 0 , rhs . 0)) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow <&'a OrderedFloat <$ exp >> for OrderedFloat <$ base > { type Output = OrderedFloat <$ base >; # [inline] fn pow (self , rhs : &'a OrderedFloat <$ exp >) -> OrderedFloat <$ base > { OrderedFloat (<$ base >:: pow (self . 0 , rhs . 0)) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow < OrderedFloat <$ exp >> for &'a OrderedFloat <$ base > { type Output = OrderedFloat <$ base >; # [inline] fn pow (self , rhs : OrderedFloat <$ exp >) -> OrderedFloat <$ base > { OrderedFloat (<$ base >:: pow (self . 0 , rhs . 0)) } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a , 'b > Pow <&'a OrderedFloat <$ exp >> for &'b OrderedFloat <$ base > { type Output = OrderedFloat <$ base >; # [inline] fn pow (self , rhs : &'a OrderedFloat <$ exp >) -> OrderedFloat <$ base > { OrderedFloat (<$ base >:: pow (self . 0 , rhs . 0)) } } } ; }
};
}
