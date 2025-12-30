// Generated macro for impl_68 (impl)
macro_rules! Depcrate_montgomeryimpl_68 {
() => {
// Module: crate::montgomery
// Provides: {"impl_68"}
// Dependencies: {}
impl ProjectiveMontgomeryPoint { # [doc = " The identity element of the group: the point at infinity."] pub fn identity () -> ProjectiveMontgomeryPoint { ProjectiveMontgomeryPoint { U : FieldElement :: ONE , W : FieldElement :: ZERO , } } # [doc = " Convert the point to affine form"] pub fn to_affine (& self) -> MontgomeryPoint { let x = self . U * self . W . invert () ; MontgomeryPoint (x . to_bytes ()) } }
};
}
