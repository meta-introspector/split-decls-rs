// Generated macro for DualQuaternion (struct)
macro_rules! Depcrate_geometry_dual_quaternionDualQuaternion {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"DualQuaternion"}
// Dependencies: {}
# [doc = " A dual quaternion."] # [doc = ""] # [doc = " # Indexing"] # [doc = ""] # [doc = " `DualQuaternions` are stored as \\[..real, ..dual\\]."] # [doc = " Both of the quaternion components are laid out in `i, j, k, w` order."] # [doc = ""] # [doc = " # Example"] # [doc = " ```"] # [doc = " # use nalgebra::{DualQuaternion, Quaternion};"] # [doc = ""] # [doc = " let real = Quaternion::new(1.0, 2.0, 3.0, 4.0);"] # [doc = " let dual = Quaternion::new(5.0, 6.0, 7.0, 8.0);"] # [doc = ""] # [doc = " let dq = DualQuaternion::from_real_and_dual(real, dual);"] # [doc = " assert_eq!(dq[0], 2.0);"] # [doc = " assert_eq!(dq[1], 3.0);"] # [doc = ""] # [doc = " assert_eq!(dq[4], 6.0);"] # [doc = " assert_eq!(dq[7], 5.0);"] # [doc = " ```"] # [doc = ""] # [doc = " NOTE:"] # [doc = "  As of December 2020, dual quaternion support is a work in progress."] # [doc = "  If a feature that you need is missing, feel free to open an issue or a PR."] # [doc = "  See <https://github.com/dimforge/nalgebra/issues/487>"] # [repr (C)] # [derive (Debug , Copy , Clone)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "DualQuaternion<T::Archived>" , bound (archive = "
        T: rkyv::Archive,
        Quaternion<T>: rkyv::Archive<Archived = Quaternion<T::Archived>>
    ")))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] pub struct DualQuaternion < T > { # [doc = " The real component of the quaternion"] pub real : Quaternion < T > , # [doc = " The dual component of the quaternion"] pub dual : Quaternion < T > , }
};
}
