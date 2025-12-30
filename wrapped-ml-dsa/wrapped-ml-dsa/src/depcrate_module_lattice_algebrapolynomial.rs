// Generated macro for Polynomial (struct)
macro_rules! Depcrate_module_lattice_algebraPolynomial {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"Polynomial"}
// Dependencies: {}
# [doc = " A `Polynomial` is a member of the ring `R_q = Z_q[X] / (X^256)` of degree-256 polynomials"] # [doc = " over the finite field with prime order `q`.  Polynomials can be added, subtracted, negated,"] # [doc = " and multiplied by field elements.  We do not define multiplication of polynomials here."] # [derive (Clone , Default , Debug , PartialEq)] pub struct Polynomial < F : Field > (pub Array < Elem < F > , U256 >) ;
};
}
