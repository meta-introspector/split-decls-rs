// Generated macro for DataType (trait)
macro_rules! Depcrate_methodtypeDataType {
() => {
// Module: crate::methodtype
// Provides: {"DataType"}
// Dependencies: {}
# [doc = " Associated data for different objects in a tree."] # [doc = ""] # [doc = " These currently require a debug bound, due to https://github.com/rust-lang/rust/issues/31518"] pub trait DataType : Sized + Default { # [doc = " Type of associated data on the Tree."] type Tree : fmt :: Debug ; # [doc = " Type of associated data on every ObjectPath."] type ObjectPath : fmt :: Debug ; # [doc = " Type of associated data on every Property."] type Property : fmt :: Debug ; # [doc = " Type of associated data on every Interface."] type Interface : fmt :: Debug ; # [doc = " Type of associated data on every Method."] type Method : fmt :: Debug ; # [doc = " Type of associated data on every Signal."] type Signal : fmt :: Debug ; }
};
}
