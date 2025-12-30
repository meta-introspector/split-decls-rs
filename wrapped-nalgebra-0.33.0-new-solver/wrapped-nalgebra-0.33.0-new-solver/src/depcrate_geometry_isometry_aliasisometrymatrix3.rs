// Generated macro for IsometryMatrix3 (type)
macro_rules! Depcrate_geometry_isometry_aliasIsometryMatrix3 {
() => {
// Module: crate::geometry::isometry_alias
// Provides: {"IsometryMatrix3"}
// Dependencies: {}
# [doc = " A 3-dimensional direct isometry using a rotation matrix for its rotational part."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Isometry`](crate::Isometry) type too.**"] # [doc = ""] # [doc = " Also known as a rigid-body motion, or as an element of SE(3)."] pub type IsometryMatrix3 < T > = Isometry < T , Rotation3 < T > , 3 > ;
};
}
