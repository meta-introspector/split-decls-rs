// Generated macro for impl_signed_mulo (macro)
macro_rules! Depcrate_int_mulimpl_signed_mulo {
() => {
// Module: crate::int::mul
// Provides: {"impl_signed_mulo"}
// Dependencies: {}
macro_rules ! impl_signed_mulo { ($ fn : ident , $ iD : ident , $ uD : ident) => { fn $ fn (lhs : $ iD , rhs : $ iD) -> ($ iD , bool) { let mut lhs = lhs ; let mut rhs = rhs ; if lhs == 0 || rhs == 0 { return (0 , false) ; } let lhs_neg = lhs < 0 ; let rhs_neg = rhs < 0 ; if lhs_neg { lhs = lhs . wrapping_neg () ; } if rhs_neg { rhs = rhs . wrapping_neg () ; } let mul_neg = lhs_neg != rhs_neg ; let (mul , o) = (lhs as $ uD) . mulo (rhs as $ uD) ; let mut mul = mul as $ iD ; if mul_neg { mul = mul . wrapping_neg () ; } if (mul < 0) != mul_neg { (mul , true) } else { (mul , o) } } } ; }
};
}
