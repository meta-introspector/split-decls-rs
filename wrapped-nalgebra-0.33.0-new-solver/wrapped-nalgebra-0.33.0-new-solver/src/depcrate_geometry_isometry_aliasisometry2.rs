// Generated macro for Isometry2 (type)
macro_rules! Depcrate_geometry_isometry_aliasIsometry2 {
() => {
// Module: crate::geometry::isometry_alias
// Provides: {"Isometry2"}
// Dependencies: {}
# [doc = " A 2-dimensional direct isometry using a unit complex number for its rotational part."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Isometry`](crate::Isometry) type too.**"] # [doc = ""] # [doc = " Also known as a 2D rigid-body motion, or as an element of SE(2)."] pub type Isometry2 < T > = Isometry < T , UnitComplex < T > , 2 > ;
};
}
