// Generated macro for shared_function (function)
macro_rules! Depcrate_encodeshared_function {
() => {
// Module: crate::encode
// Provides: {"shared_function"}
// Dependencies: {}
fn shared_function < 'a > (func : & 'a ast :: Function , _intern : & 'a Interner) -> Function < 'a > { let args = func . arguments . iter () . enumerate () . map (| (idx , arg) | FunctionArgumentData { name : arg . js_name . clone () . unwrap_or (if let syn :: Pat :: Ident (x) = & * arg . pat_type . pat { x . ident . unraw () . to_string () } else { format ! ("arg{}" , idx) } ,) , ty_override : arg . js_type . as_deref () , desc : arg . desc . as_deref () , }) . collect :: < Vec < _ > > () ; Function { args , asyncness : func . r#async , name : & func . name , generate_typescript : func . generate_typescript , generate_jsdoc : func . generate_jsdoc , variadic : func . variadic , ret_ty_override : func . ret . as_ref () . and_then (| v | v . js_type . as_deref ()) , ret_desc : func . ret . as_ref () . and_then (| v | v . desc . as_deref ()) , } }
};
}
