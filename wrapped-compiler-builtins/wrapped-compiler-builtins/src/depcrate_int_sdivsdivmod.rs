// Generated macro for sdivmod (macro)
macro_rules! Depcrate_int_sdivsdivmod {
() => {
// Module: crate::int::sdiv
// Provides: {"sdivmod"}
// Dependencies: {}
macro_rules ! sdivmod { ($ unsigned_fn : ident , $ signed_fn : ident , $ uX : ident , $ iX : ident , $ ($ attr : tt) ,*) => { intrinsics ! { $ (# [$ attr]) * # [doc = " Returns `n / d` and sets `*rem = n % d`"] pub extern "C" fn $ signed_fn (a : $ iX , b : $ iX , rem : & mut $ iX) -> $ iX { let a_neg = a < 0 ; let b_neg = b < 0 ; let mut a = a ; let mut b = b ; if a_neg { a = a . wrapping_neg () ; } if b_neg { b = b . wrapping_neg () ; } let mut r = * rem as $ uX ; let t = $ unsigned_fn (a as $ uX , b as $ uX , Some (& mut r)) as $ iX ; let mut r = r as $ iX ; if a_neg { r = r . wrapping_neg () ; } * rem = r ; if a_neg != b_neg { t . wrapping_neg () } else { t } } } } }
};
}
