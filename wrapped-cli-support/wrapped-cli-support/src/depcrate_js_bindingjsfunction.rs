// Generated macro for JsFunction (struct)
macro_rules! Depcrate_js_bindingJsFunction {
() => {
// Module: crate::js::binding
// Provides: {"JsFunction"}
// Dependencies: {}
pub struct JsFunction { pub code : String , pub ts_sig : String , pub js_doc : String , pub ts_doc : String , pub ts_arg_tys : Vec < String > , pub ts_ret_ty : Option < String > , pub ts_refs : HashSet < TsReference > , # [doc = " Whether this function has a single optional argument."] # [doc = ""] # [doc = " If the function is a setter, that means that the field it sets is optional."] pub might_be_optional_field : bool , pub catch : bool , pub log_error : bool , }
};
}
