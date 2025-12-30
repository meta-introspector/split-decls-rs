// Generated macro for NttPolynomial (struct)
macro_rules! Depcrate_module_lattice_algebraNttPolynomial {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"NttPolynomial"}
// Dependencies: {}
# [doc = " An `NttPolynomial` is a member of the NTT algebra `T_q = Z_q[X]^256` of 256-tuples of field"] # [doc = " elements.  NTT polynomials can be added and"] # [doc = " subtracted, negated, and multiplied by scalars."] # [doc = " We do not define multiplication of NTT polynomials here.  We also do not define the"] # [doc = " mappings between normal polynomials and NTT polynomials (i.e., between `R_q` and `T_q`)."] # [derive (Clone , Default , Debug , PartialEq)] pub (crate) struct NttPolynomial < F : Field > (pub Array < Elem < F > , U256 >) ;
};
}
