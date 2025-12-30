// Generated macro for IsometryMatrix2 (type)
macro_rules! Depcrate_geometry_isometry_aliasIsometryMatrix2 {
() => {
// Module: crate::geometry::isometry_alias
// Provides: {"IsometryMatrix2"}
// Dependencies: {}
# [doc = " A 2-dimensional direct isometry using a rotation matrix for its rotational part."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Isometry`](crate::Isometry) type too.**"] # [doc = ""] # [doc = " Also known as a rigid-body motion, or as an element of SE(2)."] pub type IsometryMatrix2 < T > = Isometry < T , Rotation2 < T > , 2 > ;
};
}
