// Generated macro for PropName (struct)
macro_rules! Depcrate_prop_namePropName {
() => {
// Module: crate::prop_name
// Provides: {"PropName"}
// Dependencies: {}
# [doc = " A borrowed name of a RocksDB property."] # [doc = ""] # [doc = " The value is guaranteed to be a nul-terminated UTF-8 string. This means it"] # [doc = " can be converted to [`CStr`] and [`str`] at zero cost."] # [derive (PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct PropName (CStr) ;
};
}
