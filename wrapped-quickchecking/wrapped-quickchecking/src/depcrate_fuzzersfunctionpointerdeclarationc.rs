// Generated macro for FunctionPointerDeclarationC (struct)
macro_rules! Depcrate_fuzzersFunctionPointerDeclarationC {
() => {
// Module: crate::fuzzers
// Provides: {"FunctionPointerDeclarationC"}
// Dependencies: {}
# [doc = " `FunctionPointerDeclarationC` is used in generation of C headers to represent"] # [doc = " the definition of a function pointer type."] # [derive (Debug , Clone)] pub struct FunctionPointerDeclarationC { # [doc = " The function's type qualifier, i.e. `const`."] pub type_qualifier : TypeQualifierC , # [doc = " The function's return type, i.e. `int`."] pub type_name : BaseTypeC , # [doc = " The function's pointer level, i.e. `***`."] pub pointer_level : PointerLevelC , # [doc = " The function's parameters."] pub params : ParameterListC , # [doc = " The declaration's identifier, i.e. `func_ptr_N`."] pub ident_id : String , }
};
}
