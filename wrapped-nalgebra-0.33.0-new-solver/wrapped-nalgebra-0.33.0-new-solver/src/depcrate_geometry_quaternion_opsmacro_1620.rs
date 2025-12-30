// Generated macro for macro_1620 (macro)
macro_rules! Depcrate_geometry_quaternion_opsmacro_1620 {
() => {
// Module: crate::geometry::quaternion_ops
// Provides: {"macro_1620"}
// Dependencies: {}
quaternion_op_impl ! (Mul , mul ; ; self : &'a Quaternion < T >, rhs : &'b Quaternion < T >, Output = Quaternion < T >; Quaternion :: new (self [3] . clone () * rhs [3] . clone () - self [0] . clone () * rhs [0] . clone () - self [1] . clone () * rhs [1] . clone () - self [2] . clone () * rhs [2] . clone () , self [3] . clone () * rhs [0] . clone () + self [0] . clone () * rhs [3] . clone () + self [1] . clone () * rhs [2] . clone () - self [2] . clone () * rhs [1] . clone () , self [3] . clone () * rhs [1] . clone () - self [0] . clone () * rhs [2] . clone () + self [1] . clone () * rhs [3] . clone () + self [2] . clone () * rhs [0] . clone () , self [3] . clone () * rhs [2] . clone () + self [0] . clone () * rhs [1] . clone () - self [1] . clone () * rhs [0] . clone () + self [2] . clone () * rhs [3] . clone ()) ; 'a , 'b) ;
};
}
