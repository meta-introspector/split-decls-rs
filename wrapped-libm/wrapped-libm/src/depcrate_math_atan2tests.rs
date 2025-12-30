// Generated macro for tests (module)
macro_rules! Depcrate_math_atan2tests {
() => {
// Module: crate::math::atan2
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] # [cfg_attr (x86_no_sse , ignore = "FIXME(i586): possible incorrect rounding")] fn sanity_check () { assert_eq ! (atan2 (0.0 , 1.0) , 0.0) ; assert_eq ! (atan2 (0.0 , - 1.0) , PI) ; assert_eq ! (atan2 (- 0.0 , - 1.0) , - PI) ; assert_eq ! (atan2 (3.0 , 2.0) , atan (3.0 / 2.0)) ; assert_eq ! (atan2 (2.0 , - 1.0) , atan (2.0 / - 1.0) + PI) ; assert_eq ! (atan2 (- 2.0 , - 1.0) , atan (- 2.0 / - 1.0) - PI) ; } }
};
}
