// Generated macro for define_add_variants (macro)
macro_rules! Depcrate_macrosdefine_add_variants {
() => {
// Module: crate::macros
// Provides: {"define_add_variants"}
// Dependencies: {}
# [doc = " Define borrow and non-borrow variants of `Add`."] macro_rules ! define_add_variants { ($ (GENERIC = $ generic : ident : $ bound : ident ,) ? LHS = $ lhs : ty , RHS = $ rhs : ty , Output = $ out : ty) => { impl <'b $ (, $ generic : $ bound) ?> Add <&'b $ rhs > for $ lhs { type Output = $ out ; fn add (self , rhs : &'b $ rhs) -> $ out { & self + rhs } } impl <'a $ (, $ generic : $ bound) ?> Add <$ rhs > for &'a $ lhs { type Output = $ out ; fn add (self , rhs : $ rhs) -> $ out { self + & rhs } } impl $ (<$ generic : $ bound >) ? Add <$ rhs > for $ lhs { type Output = $ out ; fn add (self , rhs : $ rhs) -> $ out { & self + & rhs } } } ; }
};
}
