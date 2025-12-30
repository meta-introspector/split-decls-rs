// Generated macro for tests (module)
macro_rules! Depcrate_acceleration_structure_typestests {
() => {
// Module: crate::acceleration_structure_types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (feature = "alloc")] mod tests { use alloc :: string :: ToString ; use objc2 :: encode :: Encode ; use crate :: MTLPackedFloat4x3 ; # [test] fn test_packed_float () { assert_eq ! (MTLPackedFloat4x3 :: ENCODING . to_string () , "{_MTLPackedFloat4x3=[4{_MTLPackedFloat3=(?={?=fff}[3f])}]}" ,) ; } }
};
}
