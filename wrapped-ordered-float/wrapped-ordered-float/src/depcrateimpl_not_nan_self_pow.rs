// Generated macro for impl_not_nan_self_pow (macro)
macro_rules! Depcrateimpl_not_nan_self_pow {
() => {
// Module: crate
// Provides: {"impl_not_nan_self_pow"}
// Dependencies: {}
macro_rules ! impl_not_nan_self_pow { ($ base : ty , $ exp : ty) => { # [cfg (any (feature = "std" , feature = "libm"))] impl Pow < NotNan <$ exp >> for NotNan <$ base > { type Output = NotNan <$ base >; # [inline] fn pow (self , rhs : NotNan <$ exp >) -> NotNan <$ base > { NotNan :: new (self . 0 . pow (rhs . 0)) . expect ("Pow resulted in NaN") } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow <&'a NotNan <$ exp >> for NotNan <$ base > { type Output = NotNan <$ base >; # [inline] fn pow (self , rhs : &'a NotNan <$ exp >) -> NotNan <$ base > { NotNan :: new (self . 0 . pow (rhs . 0)) . expect ("Pow resulted in NaN") } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow < NotNan <$ exp >> for &'a NotNan <$ base > { type Output = NotNan <$ base >; # [inline] fn pow (self , rhs : NotNan <$ exp >) -> NotNan <$ base > { NotNan :: new (self . 0 . pow (rhs . 0)) . expect ("Pow resulted in NaN") } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a , 'b > Pow <&'a NotNan <$ exp >> for &'b NotNan <$ base > { type Output = NotNan <$ base >; # [inline] fn pow (self , rhs : &'a NotNan <$ exp >) -> NotNan <$ base > { NotNan :: new (self . 0 . pow (rhs . 0)) . expect ("Pow resulted in NaN") } } } ; }
};
}
