// Generated macro for ShapeSet (struct)
macro_rules! Depcrate_util_shapeShapeSet {
() => {
// Module: crate::util::shape
// Provides: {"ShapeSet"}
// Dependencies: {}
# [doc = " A set of [`Shape`] values, which correctly handles the relationship between"] # [doc = " [newtype](Shape#variant.Newtype) and [tuple](Shape#variant.Tuple) shapes."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust"] # [doc = " # use darling_core::util::{Shape, ShapeSet};"] # [doc = " let shape_set = ShapeSet::new(vec![Shape::Tuple]);"] # [doc = ""] # [doc = " // This is correct, because all newtypes are single-field tuples."] # [doc = " assert!(shape_set.contains(&Shape::Newtype));"] # [doc = " ```"] # [derive (Debug , Clone , Default)] pub struct ShapeSet { newtype : bool , named : bool , tuple : bool , unit : bool , }
};
}
