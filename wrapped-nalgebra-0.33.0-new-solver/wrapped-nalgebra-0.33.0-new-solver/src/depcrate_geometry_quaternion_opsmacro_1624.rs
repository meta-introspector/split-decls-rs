// Generated macro for macro_1624 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1624 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1624"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a UnitQuaternion < T >, rhs : &'b UnitQuaternion < T >, Output = UnitQuaternion < T >; UnitQuaternion :: new_unchecked (self . quaternion () * rhs . quaternion ()) ; 'a , 'b) ;
};
}
