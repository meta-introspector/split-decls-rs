// Generated macro for BasicTypeDeclarationC (struct)
macro_rules! Depcrate_fuzzersBasicTypeDeclarationC {
() => {
// Module: crate::fuzzers
// Provides: {"BasicTypeDeclarationC"}
// Dependencies: {}
# [doc = " `BasicTypeDeclarationC` is used in generation of C headers to represent"] # [doc = " declarations outside of function pointers that take the form"] # [doc = " `BaseTypeC` + `TypeQualifierC` + `PointerLevelC` + `ident_id`."] # [derive (Debug , Clone)] pub struct BasicTypeDeclarationC { # [doc = " The declaration's base type, i.e. `int`."] pub type_name : BaseTypeC , # [doc = " The declaration's type qualifier, i.e. `const`."] pub type_qualifier : TypeQualifierC , # [doc = " The declaration's pointer level, i.e. `***`."] pub pointer_level : PointerLevelC , # [doc = " The declaration's array dimension, i.e. [][][]."] pub array_dimension : ArrayDimensionC , # [doc = " The declaration's identifier, i.e. `ident_N`."] pub ident_id : String , }
};
}
