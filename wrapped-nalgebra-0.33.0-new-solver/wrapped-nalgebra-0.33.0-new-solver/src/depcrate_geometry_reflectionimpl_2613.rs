// Generated macro for impl_2613 (impl)
macro_rules! Depcrate_geometry_reflectionimpl_2613 {
() => {
// Module: crate::geometry::reflection
// Provides: {"impl_2613"}
// Dependencies: {}
impl < T : ComplexField , S : Storage < T , Const < D > > , const D : usize > Reflection < T , Const < D > , S > { # [doc = " Creates a new reflection wrt. the plane orthogonal to the given axis and that contains the"] # [doc = " point `pt`."] pub fn new_containing_point (axis : Unit < Vector < T , Const < D > , S > > , pt : & Point < T , D >) -> Self { let bias = axis . dotc (& pt . coords) ; Self :: new (axis , bias) } }
};
}
