// Generated macro for Scale (struct)
macro_rules! Depcrate_geometry_scaleScale {
() => {
// Module: crate::geometry::scale
// Provides: {"Scale"}
// Dependencies: {}
# [doc = " A scale which supports non-uniform scaling."] # [repr (C)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Scale<T::Archived, D>" , bound (archive = "
        T: rkyv::Archive,
        SVector<T, D>: rkyv::Archive<Archived = SVector<T::Archived, D>>
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] # [derive (Copy , Clone)] pub struct Scale < T , const D : usize > { # [doc = " The scale coordinates, i.e., how much is multiplied to a point's coordinates when it is"] # [doc = " scaled."] pub vector : SVector < T , D > , }
};
}
