// Generated macro for define_mul_variants (macro)
macro_rules! Depcrate_macrosdefine_mul_variants {
() => {
// Module: crate::macros
// Provides: {"define_mul_variants"}
// Dependencies: {}
# [doc = " Define borrow and non-borrow variants of `Mul`."] macro_rules ! define_mul_variants { ($ (GENERIC = $ generic : ident : $ bound : ident ,) ? LHS = $ lhs : ty , RHS = $ rhs : ty , Output = $ out : ty) => { impl <'b $ (, $ generic : $ bound) ?> Mul <&'b $ rhs > for $ lhs { type Output = $ out ; fn mul (self , rhs : &'b $ rhs) -> $ out { & self * rhs } } impl <'a $ (, $ generic : $ bound) ?> Mul <$ rhs > for &'a $ lhs { type Output = $ out ; fn mul (self , rhs : $ rhs) -> $ out { self * & rhs } } impl $ (<$ generic : $ bound >) ? Mul <$ rhs > for $ lhs { type Output = $ out ; fn mul (self , rhs : $ rhs) -> $ out { & self * & rhs } } } ; }
};
}
