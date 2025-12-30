// Generated macro for clip (function)
macro_rules! Depcrateclip {
() => {
// Module: crate
// Provides: {"clip"}
// Dependencies: {}
# [doc = " clip unused bytes"] pub fn clip (tcx : TyCtxt < '_ > , u : u128 , ity : UintTy) -> u128 { let bits = Integer :: from_uint_ty (& tcx , ity) . size () . bits () ; let amt = 128 - bits ; (u << amt) >> amt }
};
}
