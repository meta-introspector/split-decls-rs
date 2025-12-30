// Generated macro for write_invoke_arg (function)
macro_rules! Depcrate_types_cpp_methodwrite_invoke_arg {
() => {
// Module: crate::types::cpp_method
// Provides: {"write_invoke_arg"}
// Dependencies: {}
fn write_invoke_arg (param : & Param) -> TokenStream { let name = param . write_ident () ; if param . is_input () && param . is_interface () { quote ! { core :: mem :: transmute_copy (&# name) } } else if (! param . is_pointer () && param . is_interface ()) || (param . is_input () && ! param . is_primitive ()) { quote ! { core :: mem :: transmute (&# name) } } else { quote ! { core :: mem :: transmute_copy (&# name) } } }
};
}
