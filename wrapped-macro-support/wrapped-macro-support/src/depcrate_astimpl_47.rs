// Generated macro for impl_47 (impl)
macro_rules! Depcrate_astimpl_47 {
() => {
// Module: crate::ast
// Provides: {"impl_47"}
// Dependencies: {}
impl Export { # [doc = " Mangles a rust -> javascript export, so that the created Ident will be unique over function"] # [doc = " name and class name, if the function belongs to a javascript class."] pub (crate) fn rust_symbol (& self) -> Ident { let mut generated_name = String :: from ("__wasm_bindgen_generated") ; if let Some (class) = & self . js_class { generated_name . push ('_') ; generated_name . push_str (class) ; } generated_name . push ('_') ; generated_name . push_str (& self . function . name . to_string ()) ; Ident :: new (& generated_name , Span :: call_site ()) } # [doc = " This is the name of the shim function that gets exported and takes the raw"] # [doc = " ABI form of its arguments and converts them back into their normal,"] # [doc = " \"high level\" form before calling the actual function."] pub (crate) fn export_name (& self) -> String { let fn_name = self . function . name . to_string () ; match & self . js_class { Some (class) => shared :: struct_function_export_name (class , & fn_name) , None => shared :: free_function_export_name (& fn_name) , } } }
};
}
