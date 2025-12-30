// Generated macro for PropertyName (struct)
macro_rules! Depcrate_prop_namePropertyName {
() => {
// Module: crate::prop_name
// Provides: {"PropertyName"}
// Dependencies: {}
# [doc = " An owned name of a RocksDB property."] # [doc = ""] # [doc = " The value is guaranteed to be a nul-terminated UTF-8 string. This means it"] # [doc = " can be converted to [`CString`] and [`String`] at zero cost."] # [derive (PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct PropertyName (CString) ;
};
}
