// Generated macro for write_param (function)
macro_rules! Depcrate_types_cpp_delegatewrite_param {
() => {
// Module: crate::types::cpp_delegate
// Provides: {"write_param"}
// Dependencies: {}
fn write_param (config : & Config , param : & Param) -> TokenStream { let name = param . write_ident () ; let type_name = param . write_name (config) ; if config . sys { return quote ! { # name : # type_name , } ; } if param . is_input () { if param . is_copyable () { return quote ! { # name : # type_name , } ; } else { return quote ! { # name : windows_core :: Ref <# type_name >, } ; } } let deref = param . deref () ; if deref . is_interface () { let type_name = deref . write_name (config) ; quote ! { # name : windows_core :: OutRef <# type_name >, } } else { quote ! { # name : # type_name , } } }
};
}
