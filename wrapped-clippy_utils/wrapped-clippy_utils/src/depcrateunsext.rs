// Generated macro for unsext (function)
macro_rules! Depcrateunsext {
() => {
// Module: crate
// Provides: {"unsext"}
// Dependencies: {}
# [expect (clippy :: cast_sign_loss)] # [doc = " clip unused bytes"] pub fn unsext (tcx : TyCtxt < '_ > , u : i128 , ity : IntTy) -> u128 { let amt = 128 - int_bits (tcx , ity) ; ((u as u128) << amt) >> amt }
};
}
