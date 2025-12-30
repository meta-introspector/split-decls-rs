// Generated macro for ImportFunctionKind (enum)
macro_rules! Depcrate_astImportFunctionKind {
() => {
// Module: crate::ast
// Provides: {"ImportFunctionKind"}
// Dependencies: {}
# [doc = " The type of a function being imported"] # [cfg_attr (feature = "extra-traits" , derive (Debug , PartialEq , Eq))] # [derive (Clone)] pub enum ImportFunctionKind { # [doc = " A class method"] Method { # [doc = " The name of the class for this method, in JS"] class : String , # [doc = " The type of the class for this method, in Rust"] ty : syn :: Type , # [doc = " The kind of method this is"] kind : MethodKind , } , # [doc = " A standard function"] Normal , }
};
}
