// Generated macro for affine (module)
macro_rules! Depcrate_edwardsaffine {
() => {
// Module: crate::edwards
// Provides: {"affine"}
// Dependencies: {}
# [doc = " This module contains the code for the Goldilocks curve."] # [doc = " The goldilocks curve is the (untwisted) Edwards curve with affine equation x^2 + y^2 = 1 - 39081x^2y^2"] # [doc = " Scalar Multiplication for this curve is pre-dominantly delegated to the Twisted Edwards variation using a (doubling) isogeny"] # [doc = " Passing the point back to the Goldilocks curve using the dual-isogeny clears the cofactor."] # [doc = " The small remainder of the Scalar Multiplication is computed on the untwisted curve."] # [doc = " See <https://www.shiftleft.org/papers/isogeny/isogeny.pdf> for details"] # [doc = ""] # [doc = " This isogeny strategy does not clear the cofactor on the Goldilocks curve unless the Scalar is a multiple of 4."] # [doc = " or the point is known to be in the q-torsion subgroup."] # [doc = " Hence, one will need to multiply by the cofactor to ensure it is cleared when using the Goldilocks curve."] # [doc = " If this is a problem, one can use a different isogeny strategy (Decaf)"] pub (crate) mod affine ;
};
}
