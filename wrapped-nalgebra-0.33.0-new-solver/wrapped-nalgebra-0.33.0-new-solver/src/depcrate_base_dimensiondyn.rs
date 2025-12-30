// Generated macro for Dyn (struct)
macro_rules! Depcrate_base_dimensionDyn {
() => {
// Module: crate::base::dimension
// Provides: {"Dyn"}
// Dependencies: {}
# [doc = " Dim of dynamically-sized algebraic entities."] # [derive (Clone , Copy , Eq , PartialEq , Debug)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize))] # [cfg_attr (feature = "rkyv-serialize" , archive_attr (derive (bytecheck :: CheckBytes)))] pub struct Dyn (pub usize) ;
};
}
