// Generated macro for macro_1650 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1650 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1650"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; SB : Storage < T , Const < 3 >> ; self : UnitQuaternion < T >, rhs : &'b Vector < T , U3 , SB >, Output = Vector3 < T >; & self * rhs ; 'b) ;
};
}
