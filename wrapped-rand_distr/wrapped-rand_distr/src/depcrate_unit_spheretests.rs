// Generated macro for tests (module)
macro_rules! Depcrate_unit_spheretests {
() => {
// Module: crate::unit_sphere
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: UnitSphere ; use crate :: Distribution ; # [test] fn norm () { let mut rng = crate :: test :: rng (1) ; for _ in 0 .. 1000 { let x : [f64 ; 3] = UnitSphere . sample (& mut rng) ; assert_almost_eq ! (x [0] * x [0] + x [1] * x [1] + x [2] * x [2] , 1. , 1e-15) ; } } }
};
}
