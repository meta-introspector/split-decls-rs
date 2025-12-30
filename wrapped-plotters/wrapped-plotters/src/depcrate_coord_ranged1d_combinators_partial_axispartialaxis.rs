// Generated macro for PartialAxis (struct)
macro_rules! Depcrate_coord_ranged1d_combinators_partial_axisPartialAxis {
() => {
// Module: crate::coord::ranged1d::combinators::partial_axis
// Provides: {"PartialAxis"}
// Dependencies: {}
# [doc = " This axis decorator will make the axis partially display on the axis."] # [doc = " At some time, we want the axis only covers some part of the value."] # [doc = " This decorator will have an additional display range defined."] # [derive (Clone)] pub struct PartialAxis < R : Ranged > (R , Range < R :: ValueType >) ;
};
}
