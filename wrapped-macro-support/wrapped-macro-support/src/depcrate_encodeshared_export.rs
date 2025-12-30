// Generated macro for shared_export (function)
macro_rules! Depcrate_encodeshared_export {
() => {
// Module: crate::encode
// Provides: {"shared_export"}
// Dependencies: {}
fn shared_export < 'a > (export : & 'a ast :: Export , intern : & 'a Interner ,) -> Result < Export < 'a > , Diagnostic > { let consumed = matches ! (export . method_self , Some (ast :: MethodSelf :: ByValue)) ; let method_kind = from_ast_method_kind (& export . function , intern , & export . method_kind) ? ; Ok (Export { class : export . js_class . as_deref () , comments : export . comments . iter () . map (| s | & * * s) . collect () , consumed , function : shared_function (& export . function , intern) , js_namespace : export . js_namespace . clone () , method_kind , start : export . start , }) }
};
}
