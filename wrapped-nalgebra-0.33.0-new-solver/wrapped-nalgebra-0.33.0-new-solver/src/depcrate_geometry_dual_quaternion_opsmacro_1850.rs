// Generated macro for macro_1850 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1850 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1850"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) for SB : Storage < T , U3 > ; self : UnitDualQuaternion < T >, rhs : &'b Unit < Vector < T , U3 , SB >>, Output = Unit < Vector3 < T >> => U3 , U4 ; Unit :: new_unchecked (self * rhs . as_ref ()) ; 'b) ;
};
}
