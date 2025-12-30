// Generated macro for impl_301 (impl)
macro_rules! Depcrate_types_cpp_delegateimpl_301 {
() => {
// Module: crate::types::cpp_delegate
// Provides: {"impl_301"}
// Dependencies: {}
impl CppDelegate { pub fn type_name (& self) -> TypeName { self . def . type_name () } pub fn write_name (& self , config : & Config) -> TokenStream { self . type_name () . write (config , & []) } pub fn method (& self) -> MethodDef { self . def . methods () . find (| method | method . name () == "Invoke") . unwrap () } pub fn write_cfg (& self , config : & Config) -> TokenStream { if ! config . package { return quote ! { } ; } Cfg :: new (& self . dependencies () , config) . write (config , false) } pub fn write (& self , config : & Config) -> TokenStream { let type_name = self . def . type_name () ; let name = to_ident (type_name . name ()) ; let method = self . method () ; let signature = method . signature (type_name . namespace () , & []) ; let mut params = quote ! { } ; for param in & signature . params { params . combine (write_param (config , param)) ; } let return_sig = config . write_return_sig (method , & signature , false) ; let arches = write_arches (self . def) ; let cfg = self . write_cfg (config) ; quote ! { # arches # cfg pub type # name = Option < unsafe extern "system" fn (# params) # return_sig >; } } }
};
}
