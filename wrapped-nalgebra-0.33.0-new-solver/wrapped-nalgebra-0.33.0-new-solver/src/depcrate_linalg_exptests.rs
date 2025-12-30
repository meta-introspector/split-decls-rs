// Generated macro for tests (module)
macro_rules! Depcrate_linalg_exptests {
() => {
// Module: crate::linalg::exp
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [allow (clippy :: float_cmp)] fn one_norm () { use crate :: Matrix3 ; let m = Matrix3 :: new (- 3.0 , 5.0 , 7.0 , 2.0 , 6.0 , 4.0 , 0.0 , 2.0 , 8.0) ; assert_eq ! (super :: one_norm (& m) , 19.0) ; } }
};
}
