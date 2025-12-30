// Generated macro for function_enum (function)
macro_rules! Depcratefunction_enum {
() => {
// Module: crate
// Provides: {"function_enum"}
// Dependencies: {}
# [doc = " Populate an enum with a variant representing function. Names are in upper camel case."] # [doc = ""] # [doc = " Applied to an empty enum. Expects one attribute `#[function_enum(BaseName)]` that provides"] # [doc = " the name of the `BaseName` enum."] # [proc_macro_attribute] pub fn function_enum (attributes : pm :: TokenStream , tokens : pm :: TokenStream) -> pm :: TokenStream { let item = syn :: parse_macro_input ! (tokens as ItemEnum) ; let res = enums :: function_enum (item , attributes . into ()) ; match res { Ok (ts) => ts , Err (e) => e . into_compile_error () , } . into () }
};
}
