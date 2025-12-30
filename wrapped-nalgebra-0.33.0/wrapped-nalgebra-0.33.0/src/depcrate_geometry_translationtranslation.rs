// Generated macro for Translation (struct)
macro_rules! Depcrate_geometry_translationTranslation {
() => {
// Module: crate::geometry::translation
// Provides: {"Translation"}
// Dependencies: {}
# [doc = " A translation."] # [repr (C)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Translation<T::Archived, D>" , bound (archive = "
        T: rkyv::Archive,
        SVector<T, D>: rkyv::Archive<Archived = SVector<T::Archived, D>>
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] # [derive (Copy , Clone)] pub struct Translation < T , const D : usize > { # [doc = " The translation coordinates, i.e., how much is added to a point's coordinates when it is"] # [doc = " translated."] pub vector : SVector < T , D > , }
};
}
