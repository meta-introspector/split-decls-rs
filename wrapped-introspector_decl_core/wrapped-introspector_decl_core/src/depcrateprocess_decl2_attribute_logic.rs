// Generated macro for process_decl2_attribute_logic (function)
macro_rules! Depcrateprocess_decl2_attribute_logic {
() => {
// Module: crate
// Provides: {"process_decl2_attribute_logic"}
// Dependencies: {}
pub fn process_decl2_attribute_logic (attr : TokenStream , item : TokenStream) -> TokenStream { let args = parse_decl_args ! (attr) ; dispatch_wrap_logic ! (item , args) }
};
}
