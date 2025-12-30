// Generated macro for Similarity (struct)
macro_rules! Depcrate_geometry_similaritySimilarity {
() => {
// Module: crate::geometry::similarity
// Provides: {"Similarity"}
// Dependencies: {}
# [doc = " A similarity, i.e., an uniform scaling, followed by a rotation, followed by a translation."] # [repr (C)] # [derive (Debug , Copy , Clone)] # [cfg_attr (feature = "serde-serialize-no-std" , derive (Serialize , Deserialize))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (serialize = "T: Scalar + Serialize,
                     R: Serialize,
                     DefaultAllocator: Allocator<Const<D>>,
                     Owned<T, Const<D>>: Serialize")))] # [cfg_attr (feature = "serde-serialize-no-std" , serde (bound (deserialize = "T: Scalar + Deserialize<'de>,
                       R: Deserialize<'de>,
                       DefaultAllocator: Allocator<Const<D>>,
                       Owned<T, Const<D>>: Deserialize<'de>")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Similarity<T::Archived, R::Archived, D>" , bound (archive = "
        T: rkyv::Archive,
        R: rkyv::Archive,
        Isometry<T, R, D>: rkyv::Archive<Archived = Isometry<T::Archived, R::Archived, D>>
    ")))] pub struct Similarity < T , R , const D : usize > { # [doc = " The part of this similarity that does not include the scaling factor."] pub isometry : Isometry < T , R , D > , scaling : T , }
};
}
