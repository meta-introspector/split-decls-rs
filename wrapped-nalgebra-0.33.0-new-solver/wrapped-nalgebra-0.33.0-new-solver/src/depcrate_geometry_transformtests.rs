// Generated macro for tests (module)
macro_rules! Depcrate_geometry_transformtests {
() => {
// Module: crate::geometry::transform
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: base :: Matrix4 ; # [test] fn checks_homogeneous_invariants_of_square_identity_matrix () { assert ! (TAffine :: check_homogeneous_invariants (& Matrix4 ::< f32 >:: identity ())) ; } }
};
}
