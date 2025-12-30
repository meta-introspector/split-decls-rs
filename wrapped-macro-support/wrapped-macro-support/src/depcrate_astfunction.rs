// Generated macro for Function (struct)
macro_rules! Depcrate_astFunction {
() => {
// Module: crate::ast
// Provides: {"Function"}
// Dependencies: {}
# [doc = " Information about a function being imported or exported"] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct Function { # [doc = " The name of the function"] pub name : String , # [doc = " The span of the function's name in Rust code"] pub name_span : Span , # [doc = " The arguments to the function"] pub arguments : Vec < FunctionArgumentData > , # [doc = " The data of return type of the function"] pub ret : Option < FunctionReturnData > , # [doc = " Any custom attributes being applied to the function"] pub rust_attrs : Vec < syn :: Attribute > , # [doc = " The visibility of this function in Rust"] pub rust_vis : syn :: Visibility , # [doc = " Whether this is an `unsafe` function"] pub r#unsafe : bool , # [doc = " Whether this is an `async` function"] pub r#async : bool , # [doc = " Whether to generate a typescript definition for this function"] pub generate_typescript : bool , # [doc = " Whether to generate jsdoc documentation for this function"] pub generate_jsdoc : bool , # [doc = " Whether this is a function with a variadict parameter"] pub variadic : bool , }
};
}
