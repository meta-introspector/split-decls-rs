// Generated macro for tests (module)
macro_rules! Depcrate_unit_circletests {
() => {
// Module: crate::unit_circle
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: UnitCircle ; use crate :: Distribution ; # [test] fn norm () { let mut rng = crate :: test :: rng (1) ; for _ in 0 .. 1000 { let x : [f64 ; 2] = UnitCircle . sample (& mut rng) ; assert_almost_eq ! (x [0] * x [0] + x [1] * x [1] , 1. , 1e-15) ; } } }
};
}
