// Generated macro for method_type_encoding (function)
macro_rules! Depcrate_runtime_definemethod_type_encoding {
() => {
// Module: crate::runtime::define
// Provides: {"method_type_encoding"}
// Dependencies: {}
fn method_type_encoding (ret : & Encoding , args : & [Encoding]) -> CString { let mut types = format ! ("{ret}{}{}" , <* mut AnyObject >:: ENCODING , Sel :: ENCODING) ; for enc in args { use core :: fmt :: Write ; write ! (& mut types , "{enc}") . unwrap () ; } CString :: new (types) . unwrap () }
};
}
