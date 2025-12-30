// Generated macro for MethodErr (struct)
macro_rules! Depcrate_errorMethodErr {
() => {
// Module: crate::error
// Provides: {"MethodErr"}
// Dependencies: {}
# [derive (Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] # [doc = " A D-Bus Method Error, containing an error name and a description."] # [doc = ""] # [doc = " Unlike the \"Error\" struct, this is a Rust native struct."] pub struct MethodErr (ErrorName < 'static > , String) ;
};
}
