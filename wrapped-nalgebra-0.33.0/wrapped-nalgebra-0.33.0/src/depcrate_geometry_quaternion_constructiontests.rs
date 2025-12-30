// Generated macro for tests (module)
macro_rules! Depcrate_geometry_quaternion_constructiontests {
() => {
// Module: crate::geometry::quaternion_construction
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "rand")] mod tests { use super :: * ; use rand :: SeedableRng ; use rand_xorshift ; # [test] fn random_unit_quats_are_unit () { let mut rng = rand_xorshift :: XorShiftRng :: from_seed ([0xAB ; 16]) ; for _ in 0 .. 1000 { let x = rng . gen :: < UnitQuaternion < f32 > > () ; assert ! (relative_eq ! (x . into_inner () . norm () , 1.0)) } } }
};
}
