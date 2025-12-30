// Generated macro for Const (struct)
macro_rules! Depcrate_base_dimensionConst {
() => {
// Module: crate::base::dimension
// Provides: {"Const"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , PartialEq , Eq , Hash)] # [cfg_attr (feature = "rkyv-serialize-no-std" , derive (rkyv :: Archive , rkyv :: Serialize , rkyv :: Deserialize) , archive (as = "Self"))] # [cfg_attr (feature = "rkyv-serialize" , derive (bytecheck :: CheckBytes))] pub struct Const < const R : usize > ;
};
}
