// Generated macro for tests (module)
macro_rules! Depcrate_base_matrixtests {
() => {
// Module: crate::base::matrix
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] fn empty_display () { let vec : Vec < f64 > = Vec :: new () ; let dvector = crate :: DVector :: from_vec (vec) ; assert_eq ! (format ! ("{}" , dvector) , "[ ]") } # [test] fn lower_exp () { let test = crate :: Matrix2 :: new (1e6 , 2e5 , 2e-5 , 1.) ; assert_eq ! (format ! ("{:e}" , test) , r"
  ┌           ┐
  │  1e6  2e5 │
  │ 2e-5  1e0 │
  └           ┘

") } }
};
}
