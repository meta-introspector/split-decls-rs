// Generated macro for macro_1649 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1649 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1649"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; SB : Storage < T , Const < 3 >> ; self : &'a UnitQuaternion < T >, rhs : Vector < T , U3 , SB >, Output = Vector3 < T >; self * & rhs ; 'a) ;
};
}
