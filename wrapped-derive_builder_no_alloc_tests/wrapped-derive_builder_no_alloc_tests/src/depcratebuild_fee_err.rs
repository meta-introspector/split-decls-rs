// Generated macro for build_fee_err (function)
macro_rules! Depcratebuild_fee_err {
() => {
// Module: crate
// Provides: {"build_fee_err"}
// Dependencies: {}
pub fn build_fee_err () -> Option < FeeBuilderError > { let fee = FeeBuilder :: default () . build () ; fee . err () }
};
}
