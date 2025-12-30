// Generated macro for smod (macro)
macro_rules! Depcrate_int_sdivsmod {
() => {
// Module: crate::int::sdiv
// Provides: {"smod"}
// Dependencies: {}
macro_rules ! smod { ($ unsigned_fn : ident , $ signed_fn : ident , $ uX : ident , $ iX : ident , $ ($ attr : tt) ,*) => { intrinsics ! { $ (# [$ attr]) * # [doc = " Returns `n % d`"] pub extern "C" fn $ signed_fn (a : $ iX , b : $ iX) -> $ iX { let a_neg = a < 0 ; let b_neg = b < 0 ; let mut a = a ; let mut b = b ; if a_neg { a = a . wrapping_neg () ; } if b_neg { b = b . wrapping_neg () ; } let r = $ unsigned_fn (a as $ uX , b as $ uX) as $ iX ; if a_neg { r . wrapping_neg () } else { r } } } } }
};
}
