// Generated macro for pow_impl (macro)
macro_rules! Depcrate_powpow_impl {
() => {
// Module: crate::pow
// Provides: {"pow_impl"}
// Dependencies: {}
macro_rules ! pow_impl { ($ t : ty) => { pow_impl ! ($ t , u8) ; pow_impl ! ($ t , usize) ; } ; ($ t : ty , $ rhs : ty) => { pow_impl ! ($ t , $ rhs , usize , pow) ; } ; ($ t : ty , $ rhs : ty , $ desired_rhs : ty , $ method : expr) => { impl Pow <$ rhs > for $ t { type Output = $ t ; # [inline] fn pow (self , rhs : $ rhs) -> $ t { ($ method) (self , <$ desired_rhs >:: from (rhs)) } } impl <'a > Pow <&'a $ rhs > for $ t { type Output = $ t ; # [inline] fn pow (self , rhs : &'a $ rhs) -> $ t { ($ method) (self , <$ desired_rhs >:: from (* rhs)) } } impl <'a > Pow <$ rhs > for &'a $ t { type Output = $ t ; # [inline] fn pow (self , rhs : $ rhs) -> $ t { ($ method) (* self , <$ desired_rhs >:: from (rhs)) } } impl <'a , 'b > Pow <&'a $ rhs > for &'b $ t { type Output = $ t ; # [inline] fn pow (self , rhs : &'a $ rhs) -> $ t { ($ method) (* self , <$ desired_rhs >:: from (* rhs)) } } } ; }
};
}
