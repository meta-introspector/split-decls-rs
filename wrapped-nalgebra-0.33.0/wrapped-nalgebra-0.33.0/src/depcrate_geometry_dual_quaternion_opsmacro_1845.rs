// Generated macro for macro_1845 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1845 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1845"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : Point3 < T >, Output = Point3 < T > => U3 , U1 ; self * & rhs ; 'a) ;
};
}
