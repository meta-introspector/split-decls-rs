// Generated macro for define_sub_variants (macro)
macro_rules! Depcrate_macrosdefine_sub_variants {
() => {
// Module: crate::macros
// Provides: {"define_sub_variants"}
// Dependencies: {}
# [doc = " Define borrow and non-borrow variants of `Sub`."] macro_rules ! define_sub_variants { ($ (GENERIC = $ generic : ident : $ bound : ident ,) ? LHS = $ lhs : ty , RHS = $ rhs : ty , Output = $ out : ty) => { impl <'b $ (, $ generic : $ bound) ?> Sub <&'b $ rhs > for $ lhs { type Output = $ out ; fn sub (self , rhs : &'b $ rhs) -> $ out { & self - rhs } } impl <'a $ (, $ generic : $ bound) ?> Sub <$ rhs > for &'a $ lhs { type Output = $ out ; fn sub (self , rhs : $ rhs) -> $ out { self - & rhs } } impl $ (<$ generic : $ bound >) ? Sub <$ rhs > for $ lhs { type Output = $ out ; fn sub (self , rhs : $ rhs) -> $ out { & self - & rhs } } } ; }
};
}
