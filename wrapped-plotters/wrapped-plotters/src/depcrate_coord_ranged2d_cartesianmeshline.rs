// Generated macro for MeshLine (enum)
macro_rules! Depcrate_coord_ranged2d_cartesianMeshLine {
() => {
// Module: crate::coord::ranged2d::cartesian
// Provides: {"MeshLine"}
// Dependencies: {}
# [doc = " Represent a coordinate mesh for the two ranged value coordinate system"] pub enum MeshLine < 'a , X : Ranged , Y : Ranged > { # [doc = " Used to plot the horizontal lines of the mesh"] XMesh (BackendCoord , BackendCoord , & 'a X :: ValueType) , # [doc = " Used to plot the vertical lines of the mesh"] YMesh (BackendCoord , BackendCoord , & 'a Y :: ValueType) , }
};
}
