// Generated macro for ImportKind (enum)
macro_rules! Depcrate_astImportKind {
() => {
// Module: crate::ast
// Provides: {"ImportKind"}
// Dependencies: {}
# [doc = " The type of item being imported"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub enum ImportKind { # [doc = " Importing a function"] Function (ImportFunction) , # [doc = " Importing a static value"] Static (ImportStatic) , # [doc = " Importing a static string"] String (ImportString) , # [doc = " Importing a type/class"] Type (ImportType) , # [doc = " Importing a JS enum"] Enum (StringEnum) , }
};
}
