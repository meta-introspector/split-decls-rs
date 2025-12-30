// Generated macro for write_produce_type (function)
macro_rules! Depcrate_types_cpp_methodwrite_produce_type {
() => {
// Module: crate::types::cpp_method
// Provides: {"write_produce_type"}
// Dependencies: {}
fn write_produce_type (config : & Config , param : & Param , hint : ParamHint) -> TokenStream { let name = param . write_ident () ; let kind = param . write_default (config) ; if param . is_input () && param . is_interface () { let type_name = param . write_name (config) ; quote ! { # name : windows_core :: Ref <# type_name >, } } else if ! param . is_input () && param . deref () . is_interface () && ! hint . is_array () { let type_name = param . deref () . write_name (config) ; quote ! { # name : windows_core :: OutRef <# type_name >, } } else if param . is_input () { if param . is_primitive () { quote ! { # name : # kind , } } else { quote ! { # name : &# kind , } } } else { quote ! { # name : # kind , } } }
};
}
