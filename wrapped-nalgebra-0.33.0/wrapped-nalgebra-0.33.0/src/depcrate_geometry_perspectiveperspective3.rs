// Generated macro for Perspective3 (struct)
macro_rules! Depcrate_geometry_perspectivePerspective3 {
() => {
// Module: crate::geometry::perspective
// Provides: {"Perspective3"}
// Dependencies: {}
# [doc = " A 3D perspective projection stored as a homogeneous 4x4 matrix."] # [repr (C)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Perspective3<T::Archived>" , bound (archive = "
        T: rkyv::Archive,
        Matrix4<T>: rkyv::Archive<Archived = Matrix4<T::Archived>>
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] # [derive (Copy , Clone)] pub struct Perspective3 < T > { matrix : Matrix4 < T > , }
};
}
