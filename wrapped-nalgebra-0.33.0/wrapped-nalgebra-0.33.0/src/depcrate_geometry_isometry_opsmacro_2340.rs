// Generated macro for macro_2340 (macro)
macro_rules! Depcrate_geometry_isometry_opsmacro_2340 {
() => {
// Module: crate::geometry::isometry_ops
// Provides: {"macro_2340"}
// Dependencies: {}
isometry_from_composition_impl_all ! (Div , div ; D ; self : Isometry < T , Rotation < T , D >, D >, rhs : Rotation < T , D >, Output = Isometry < T , Rotation < T , D >, D >; [val val] => Isometry :: from_parts (self . translation , self . rotation / rhs) ; [ref val] => Isometry :: from_parts (self . translation . clone () , self . rotation . clone () / rhs) ; [val ref] => Isometry :: from_parts (self . translation , self . rotation / rhs . clone ()) ; [ref ref] => Isometry :: from_parts (self . translation . clone () , self . rotation . clone () / rhs . clone ()) ;) ;
};
}
