// Generated macro for macro_1648 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1648 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1648"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; SB : Storage < T , Const < 3 >> ; self : &'a UnitQuaternion < T >, rhs : &'b Vector < T , Const < 3 >, SB >, Output = Vector3 < T >; { let two : T = crate :: convert (2.0f64) ; let t = self . as_ref () . vector () . cross (rhs) * two ; let cross = self . as_ref () . vector () . cross (& t) ; t * self . as_ref () . scalar () + cross + rhs } ; 'a , 'b) ;
};
}
