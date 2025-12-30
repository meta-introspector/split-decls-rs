// Generated macro for impl_63 (impl)
macro_rules! Depcrate_montgomeryimpl_63 {
() => {
// Module: crate::montgomery
// Provides: {"impl_63"}
// Dependencies: {}
impl Mul < & EdwardsScalar > for & MontgomeryPoint { type Output = MontgomeryPoint ; # [allow (clippy :: suspicious_arithmetic_impl)] fn mul (self , scalar : & EdwardsScalar) -> MontgomeryPoint { let affine_u = FieldElement :: from_bytes (& self . 0) ; let mut x0 = ProjectiveMontgomeryPoint :: identity () ; let mut x1 = ProjectiveMontgomeryPoint { U : affine_u , W : FieldElement :: ONE , } ; let bits = scalar . bits () ; let mut swap = 0 ; for s in (0 .. 448) . rev () { let bit = bits [s] as u8 ; let choice : u8 = swap ^ bit ; ProjectiveMontgomeryPoint :: conditional_swap (& mut x0 , & mut x1 , Choice :: from (choice)) ; differential_add_and_double (& mut x0 , & mut x1 , & affine_u) ; swap = bit ; } x0 . to_affine () } }
};
}
