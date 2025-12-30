// Generated macro for macro_1848 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1848 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1848"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) for SB : Storage < T , U3 > ; self : &'a UnitDualQuaternion < T >, rhs : &'b Unit < Vector < T , U3 , SB >>, Output = Unit < Vector3 < T >> => U3 , U4 ; Unit :: new_unchecked (self * rhs . as_ref ()) ; 'a , 'b) ;
};
}
