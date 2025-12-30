// Generated macro for int_bits (function)
macro_rules! Depcrateint_bits {
() => {
// Module: crate
// Provides: {"int_bits"}
// Dependencies: {}
pub fn int_bits (tcx : TyCtxt < '_ > , ity : IntTy) -> u64 { Integer :: from_int_ty (& tcx , ity) . size () . bits () }
};
}
