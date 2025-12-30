// Generated macro for sext (function)
macro_rules! Depcratesext {
() => {
// Module: crate
// Provides: {"sext"}
// Dependencies: {}
# [expect (clippy :: cast_possible_wrap)] # [doc = " Turn a constant int byte representation into an i128"] pub fn sext (tcx : TyCtxt < '_ > , u : u128 , ity : IntTy) -> i128 { let amt = 128 - int_bits (tcx , ity) ; ((u as i128) << amt) >> amt }
};
}
