// Generated macro for OPoint (struct)
macro_rules! Depcrate_geometry_pointOPoint {
() => {
// Module: crate::geometry::point
// Provides: {"OPoint"}
// Dependencies: {}
# [doc = " A point in an euclidean space."] # [doc = ""] # [doc = " The difference between a point and a vector is only semantic. See [the user guide](https://www.nalgebra.org/docs/user_guide/points_and_transformations)"] # [doc = " for details on the distinction. The most notable difference that vectors ignore translations."] # [doc = " In particular, an [`Isometry2`](crate::Isometry2) or [`Isometry3`](crate::Isometry3) will"] # [doc = " transform points by applying a rotation and a translation on them. However, these isometries"] # [doc = " will only apply rotations to vectors (when doing `isometry * vector`, the translation part of"] # [doc = " the isometry is ignored)."] # [doc = ""] # [doc = " # Construction"] # [doc = " * [From individual components <span style=\"float:right;\">`new`…</span>](#construction-from-individual-components)"] # [doc = " * [Swizzling <span style=\"float:right;\">`xx`, `yxz`…</span>](#swizzling)"] # [doc = " * [Other construction methods <span style=\"float:right;\">`origin`, `from_slice`, `from_homogeneous`…</span>](#other-construction-methods)"] # [doc = ""] # [doc = " # Transformation"] # [doc = " Transforming a point by an [Isometry](crate::Isometry), [rotation](crate::Rotation), etc. can be"] # [doc = " achieved by multiplication, e.g., `isometry * point` or `rotation * point`. Some of these transformation"] # [doc = " may have some other methods, e.g., `isometry.inverse_transform_point(&point)`. See the documentation"] # [doc = " of said transformations for details."] # [repr (C)] # [derive (Clone)] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "OPoint<T::Archived, D>" , bound (archive = "
        T: rkyv::Archive,
        T::Archived: Scalar,
        OVector<T, D>: rkyv::Archive<Archived = OVector<T::Archived, D>>,
        DefaultAllocator: Allocator<D>,
    ")))] pub struct OPoint < T : Scalar , D : DimName > where DefaultAllocator : Allocator < D > , { # [doc = " The coordinates of this point, i.e., the shift from the origin."] pub coords : OVector < T , D > , }
};
}
