// Generated macro for macro_2348 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2348 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2348"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Mul , mul ; ; self : Isometry < T , UnitComplex < T >, 2 >, rhs : UnitComplex < T >, Output = Isometry < T , UnitComplex < T >, 2 >; [val val] => Isometry :: from_parts (self . translation , self . rotation * rhs) ; [ref val] => Isometry :: from_parts (self . translation . clone () , self . rotation . clone () * rhs) ; [val ref] => Isometry :: from_parts (self . translation , self . rotation * rhs . clone ()) ; [ref ref] => Isometry :: from_parts (self . translation . clone () , self . rotation . clone () * rhs . clone ()) ;) ;
};
}
