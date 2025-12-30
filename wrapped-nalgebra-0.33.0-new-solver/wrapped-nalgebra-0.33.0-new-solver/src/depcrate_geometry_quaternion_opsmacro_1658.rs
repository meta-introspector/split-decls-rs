// Generated macro for macro_1658 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1658 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1658"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; SB : Storage < T , Const < 3 >> ; self : UnitQuaternion < T >, rhs : &'b Unit < Vector < T , U3 , SB >>, Output = Unit < Vector3 < T >>; Unit :: new_unchecked (self * rhs . as_ref ()) ; 'b) ;
};
}
