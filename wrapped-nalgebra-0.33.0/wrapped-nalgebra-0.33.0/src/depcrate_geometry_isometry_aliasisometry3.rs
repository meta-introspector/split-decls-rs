// Generated macro for Isometry3 (type)
macro_rules! Depcrate_geometry_isometry_aliasIsometry3 {
() => {
// Module: crate::geometry::isometry_alias
// Provides: {"Isometry3"}
// Dependencies: {}
# [doc = " A 3-dimensional direct isometry using a unit quaternion for its rotational part."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Isometry`](crate::Isometry) type too.**"] # [doc = ""] # [doc = " Also known as a rigid-body motion, or as an element of SE(3)."] pub type Isometry3 < T > = Isometry < T , UnitQuaternion < T > , 3 > ;
};
}
