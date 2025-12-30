// Generated macro for NttMatrix (struct)
macro_rules! Depcrate_module_lattice_algebraNttMatrix {
() => {
// Module: crate::module_lattice::algebra
// Provides: {"NttMatrix"}
// Dependencies: {}
# [doc = " A K x L matrix of NTT-domain polynomials.  Each vector represents a row of the matrix, so that"] # [doc = " multiplying on the right just requires iteration.  Multiplication on the right by vectors"] # [doc = " is the only defined operation, and is only defined when multiplication of NTT polynomials"] # [doc = " is defined."] # [derive (Clone , Default , Debug , PartialEq)] pub (crate) struct NttMatrix < F : Field , K : ArraySize , L : ArraySize > (pub Array < NttVector < F , L > , K >) ;
};
}
