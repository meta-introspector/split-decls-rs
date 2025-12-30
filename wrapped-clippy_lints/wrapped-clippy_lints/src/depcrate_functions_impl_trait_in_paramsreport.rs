// Generated macro for report (function)
macro_rules! Depcrate_functions_impl_trait_in_paramsreport {
() => {
// Module: crate::functions::impl_trait_in_params
// Provides: {"report"}
// Dependencies: {}
fn report (cx : & LateContext < '_ > , param : & GenericParam < '_ > , generics : & Generics < '_ >) { span_lint_and_then (cx , IMPL_TRAIT_IN_PARAMS , param . span , "`impl Trait` used as a function parameter" , | diag | { if let Some (gen_span) = generics . span_for_param_suggestion () { diag . span_suggestion_verbose (gen_span , "add a type parameter" , format ! (", {{ /* Generic name */ }}: {}" , & param . name . ident () . as_str () [5 ..]) , Applicability :: HasPlaceholders ,) ; } else { diag . span_suggestion_verbose (generics . span , "add a type parameter" , format ! ("<{{ /* Generic name */ }}: {}>" , & param . name . ident () . as_str () [5 ..]) , Applicability :: HasPlaceholders ,) ; } } ,) ; }
};
}
