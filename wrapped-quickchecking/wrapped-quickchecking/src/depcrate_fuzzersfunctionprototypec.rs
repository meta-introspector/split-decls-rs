// Generated macro for FunctionPrototypeC (struct)
macro_rules! Depcrate_fuzzersFunctionPrototypeC {
() => {
// Module: crate::fuzzers
// Provides: {"FunctionPrototypeC"}
// Dependencies: {}
# [doc = " `FunctionPrototypeC` is used in generation of C headers to represent the"] # [doc = " definition of a function prototype."] # [derive (Debug , Clone)] pub struct FunctionPrototypeC { # [doc = " The function's type qualifier, i.e. `const`."] pub type_qualifier : TypeQualifierC , # [doc = " The function's return type, i.e. `int`."] pub type_name : BaseTypeC , # [doc = " The function's pointer level, i.e. `***`."] pub pointer_level : PointerLevelC , # [doc = " The function's parameters."] pub params : ParameterListC , # [doc = " The prototype's identifier, i.e. `func_N`."] pub ident_id : String , }
};
}
