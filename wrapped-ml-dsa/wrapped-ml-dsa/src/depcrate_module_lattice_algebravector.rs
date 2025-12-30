// Generated macro for Vector (struct)
macro_rules! Depcrate_module_lattice_algebraVector {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"Vector"}
// Dependencies: {}
# [doc = " A `Vector` is a vector of polynomials from `R_q` of length `K`.  Vectors can be"] # [doc = " added, subtracted, negated, and multiplied by field elements."] # [derive (Clone , Default , Debug , PartialEq)] pub struct Vector < F : Field , K : ArraySize > (pub Array < Polynomial < F > , K >) ;
};
}
