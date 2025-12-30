// Generated macro for macro_1844 (macro)
macro_rules! Depcrate_geometry_dual_quaternion_opsmacro_1844 {
() => {
// Module: crate::geometry::dual_quaternion_ops
// Provides: {"macro_1844"}
// Dependencies: {}
dual_quaternion_op_impl ! (Mul , mul ; (U4 , U1) , (U3 , U1) ; self : &'a UnitDualQuaternion < T >, rhs : &'b Point3 < T >, Output = Point3 < T > => U3 , U1 ; { let two : T = crate :: convert (2.0f64) ; let q_point = Quaternion :: from_parts (T :: zero () , rhs . coords . clone ()) ; Point :: from (((self . as_ref () . real . clone () * q_point + self . as_ref () . dual . clone () * two) * self . as_ref () . real . clone () . conjugate ()) . vector () . into_owned () ,) } ; 'a , 'b) ;
};
}
