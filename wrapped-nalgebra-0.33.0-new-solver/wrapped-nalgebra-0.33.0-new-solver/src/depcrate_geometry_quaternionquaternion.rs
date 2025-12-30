// Generated macro for Quaternion (struct)
macro_rules! Depcrate_geometry_quaternionQuaternion {
() => {
// Module: crate::geometry::quaternion
// Provides: {"Quaternion"}
// Dependencies: {}
# [doc = " A quaternion. See the type alias `UnitQuaternion = Unit<Quaternion>` for a quaternion"] # [doc = " that may be used as a rotation."] # [repr (C)] # [derive (Copy , Clone)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Quaternion<T::Archived>" , bound (archive = "
            T: rkyv::Archive,
            Vector4<T>: rkyv::Archive<Archived = Vector4<T::Archived>>
        ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] pub struct Quaternion < T > { # [doc = " This quaternion as a 4D vector of coordinates in the `[ x, y, z, w ]` storage order."] pub coords : Vector4 < T > , }
};
}
