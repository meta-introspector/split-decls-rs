// Generated macro for macro_2342 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2342 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2342"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; ; self : Isometry < T , UnitQuaternion < T >, 3 >, rhs : UnitQuaternion < T >, Output = Isometry < T , UnitQuaternion < T >, 3 >; [val val] => Isometry :: from_parts (self . translation , self . rotation * rhs) ; [ref val] => Isometry :: from_parts (self . translation . clone () , self . rotation . clone () * rhs) ; [val ref] => Isometry :: from_parts (self . translation , self . rotation * rhs . clone ()) ; [ref ref] => Isometry :: from_parts (self . translation . clone () , self . rotation . clone () * rhs . clone ()) ;) ;
};
}
