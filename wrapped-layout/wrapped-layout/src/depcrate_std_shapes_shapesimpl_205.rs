// Generated macro for impl_205 (impl)
macro_rules! Depcrate_std_shapes_shapesimpl_205 {
() => {
// Module: crate::std_shapes::shapes
// Provides: {"impl_205"}
// Dependencies: {}
impl ShapeKind { pub fn new_box (s : & str) -> Self { ShapeKind :: Box (s . to_string ()) } pub fn new_circle (s : & str) -> Self { ShapeKind :: Circle (s . to_string ()) } pub fn new_double_circle (s : & str) -> Self { ShapeKind :: DoubleCircle (s . to_string ()) } pub fn new_record (r : & RecordDef) -> Self { ShapeKind :: Record (r . clone ()) } pub fn new_connector (s : & str) -> Self { if s . is_empty () { return ShapeKind :: Connector (None) ; } ShapeKind :: Connector (Some (s . to_string ())) } }
};
}
