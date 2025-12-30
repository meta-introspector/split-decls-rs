// Generated macro for impl_not_nan_pow (macro)
macro_rules! Depcrateimpl_not_nan_pow {
() => {
// Module: crate
// Provides: {"impl_not_nan_pow"}
// Dependencies: {}
macro_rules ! impl_not_nan_pow { ($ inner : ty , $ rhs : ty) => { # [cfg (any (feature = "std" , feature = "libm"))] impl Pow <$ rhs > for NotNan <$ inner > { type Output = NotNan <$ inner >; # [inline] fn pow (self , rhs : $ rhs) -> NotNan <$ inner > { NotNan :: new (<$ inner >:: pow (self . 0 , rhs)) . expect ("Pow resulted in NaN") } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow <&'a $ rhs > for NotNan <$ inner > { type Output = NotNan <$ inner >; # [inline] fn pow (self , rhs : &'a $ rhs) -> NotNan <$ inner > { NotNan :: new (<$ inner >:: pow (self . 0 , * rhs)) . expect ("Pow resulted in NaN") } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a > Pow <$ rhs > for &'a NotNan <$ inner > { type Output = NotNan <$ inner >; # [inline] fn pow (self , rhs : $ rhs) -> NotNan <$ inner > { NotNan :: new (<$ inner >:: pow (self . 0 , rhs)) . expect ("Pow resulted in NaN") } } # [cfg (any (feature = "std" , feature = "libm"))] impl <'a , 'b > Pow <&'a $ rhs > for &'b NotNan <$ inner > { type Output = NotNan <$ inner >; # [inline] fn pow (self , rhs : &'a $ rhs) -> NotNan <$ inner > { NotNan :: new (<$ inner >:: pow (self . 0 , * rhs)) . expect ("Pow resulted in NaN") } } } ; }
};
}
