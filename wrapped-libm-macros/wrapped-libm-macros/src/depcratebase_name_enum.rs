// Generated macro for base_name_enum (function)
macro_rules! Depcratebase_name_enum {
() => {
// Module: crate
// Provides: {"base_name_enum"}
// Dependencies: {}
# [doc = " Create an enum representing all possible base names, with names in upper camel case."] # [doc = ""] # [doc = " Applied to an empty enum."] # [proc_macro_attribute] pub fn base_name_enum (attributes : pm :: TokenStream , tokens : pm :: TokenStream) -> pm :: TokenStream { let item = syn :: parse_macro_input ! (tokens as ItemEnum) ; let res = enums :: base_name_enum (item , attributes . into ()) ; match res { Ok (ts) => ts , Err (e) => e . into_compile_error () , } . into () }
};
}
