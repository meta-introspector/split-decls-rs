// Generated macro for Orthographic3 (struct)
macro_rules! Depcrate_geometry_orthographicOrthographic3 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"Orthographic3"}
// Dependencies: {}
# [doc = " A 3D orthographic projection stored as a homogeneous 4x4 matrix."] # [repr (C)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Orthographic3<T::Archived>" , bound (archive = "
        T: rkyv::Archive,
        Matrix4<T>: rkyv::Archive<Archived = Matrix4<T::Archived>>
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] # [derive (Copy , Clone)] pub struct Orthographic3 < T > { matrix : Matrix4 < T > , }
};
}
