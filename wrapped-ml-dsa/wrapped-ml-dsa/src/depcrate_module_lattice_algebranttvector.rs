// Generated macro for NttVector (struct)
macro_rules! Depcrate_module_lattice_algebraNttVector {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"NttVector"}
// Dependencies: {}
# [doc = " An `NttVector` is a vector of polynomials from `T_q` of length `K`.  NTT vectors can be"] # [doc = " added and subtracted.  If multiplication is defined for NTT polynomials, then NTT vectors"] # [doc = " can be multiplied by NTT polynomials, and \"multiplied\" with each other to produce a dot"] # [doc = " product."] # [derive (Clone , Default , Debug , PartialEq)] pub (crate) struct NttVector < F : Field , K : ArraySize > (pub Array < NttPolynomial < F > , K >) ;
};
}
