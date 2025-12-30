// Generated macro for call_script_helper (function)
macro_rules! Depcrate_helpers_scriptingcall_script_helper {
() => {
// Module: crate::helpers::scripting
// Provides: {"call_script_helper"}
// Dependencies: {}
# [inline] fn call_script_helper < 'reg : 'rc , 'rc > (params : & [PathAndJson < 'rc >] , hash : & BTreeMap < & 'reg str , PathAndJson < 'rc > > , engine : & Engine , script : & AST ,) -> Result < ScopedJson < 'rc > , RenderError > { let params : Dynamic = to_dynamic (params . iter () . map (| p | p . value ()) . collect :: < Vec < & Json > > ()) . map_err (RenderErrorReason :: from) ? ; let hash : Dynamic = to_dynamic (hash . iter () . map (| (k , v) | ((* k) . to_owned () , v . value ())) . collect :: < HashMap < String , & Json > > () ,) . map_err (RenderErrorReason :: from) ? ; let mut scope = Scope :: new () ; scope . push_dynamic ("params" , params) ; scope . push_dynamic ("hash" , hash) ; let result = engine . eval_ast_with_scope :: < Dynamic > (& mut scope , script) . map_err (RenderErrorReason :: from) ? ; let result_json : Json = from_dynamic (& result) . map_err (RenderErrorReason :: from) ? ; Ok (ScopedJson :: Derived (result_json)) }
};
}
