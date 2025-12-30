// Generated macro for macro_1780 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1780 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1780"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U4 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b UnitDualQuaternion < T >, Output = UnitDualQuaternion < T >; UnitDualQuaternion :: new_unchecked (self . as_ref () * rhs . as_ref ()) ; 'a , 'b) ;
};
}
