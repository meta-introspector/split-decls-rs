// Generated macro for shared_import_function (function)
macro_rules! Depcrate_encodeshared_import_function {
() => {
// Module: crate::encode
// Provides: {"shared_import_function"}
// Dependencies: {}
fn shared_import_function < 'a > (i : & 'a ast :: ImportFunction , intern : & 'a Interner ,) -> Result < ImportFunction < 'a > , Diagnostic > { let method = match & i . kind { ast :: ImportFunctionKind :: Method { class , kind , .. } => { let kind = from_ast_method_kind (& i . function , intern , kind) ? ; Some (MethodData { class , kind }) } ast :: ImportFunctionKind :: Normal => None , } ; Ok (ImportFunction { shim : intern . intern (& i . shim) , catch : i . catch , method , assert_no_shim : i . assert_no_shim , structural : i . structural , function : shared_function (& i . function , intern) , variadic : i . variadic , }) }
};
}
