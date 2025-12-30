// Generated macro for join (function)
macro_rules! Depcrate_ty_type_certainty_certaintyjoin {
() => {
// Module: crate::ty::type_certainty::certainty
// Provides: {"join"}
// Dependencies: {}
# [doc = " Think: `iter.any(/* is certain */)`"] pub fn join (iter : impl Iterator < Item = Certainty >) -> Certainty { iter . fold (Certainty :: Uncertain , Certainty :: join) }
};
}
