// Generated macro for struct_name (function)
macro_rules! Depcrate_property_test_codegenstruct_name {
() => {
// Module: crate::property_test::codegen
// Provides: {"struct_name"}
// Dependencies: {}
# [doc = " Convert the name of a function to the name of a struct representing its args"] # [doc = ""] # [doc = " E.g. `some_function` -> `SomeFunctionArgs`"] fn struct_name (fn_name : & Ident) -> Ident { use convert_case :: { Case , Casing } ; let name = fn_name . to_string () ; let name = name . to_case (Case :: Pascal) ; let name = format ! ("{name}Args") ; Ident :: new (& name , fn_name . span ()) }
};
}
