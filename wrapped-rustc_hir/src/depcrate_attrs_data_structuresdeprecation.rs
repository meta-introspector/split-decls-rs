// Generated macro for Deprecation (struct)
macro_rules! Depcrate_attrs_data_structuresDeprecation {
() => {
// Module: crate::attrs::data_structures
// Provides: {"Deprecation"}
// Dependencies: {}
# [derive (Copy , Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub struct Deprecation { pub since : DeprecatedSince , # [doc = " The note to issue a reason."] pub note : Option < Symbol > , # [doc = " A text snippet used to completely replace any use of the deprecated item in an expression."] # [doc = ""] # [doc = " This is currently unstable."] pub suggestion : Option < Symbol > , }
};
}
