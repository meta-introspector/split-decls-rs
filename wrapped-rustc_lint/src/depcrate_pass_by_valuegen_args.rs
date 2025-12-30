// Generated macro for gen_args (function)
macro_rules! Depcrate_pass_by_valuegen_args {
() => {
// Module: crate::pass_by_value
// Provides: {"gen_args"}
// Dependencies: {}
fn gen_args (cx : & LateContext < '_ > , segment : & PathSegment < '_ >) -> String { if let Some (args) = & segment . args { let params = args . args . iter () . map (| arg | match arg { GenericArg :: Lifetime (lt) => lt . to_string () , GenericArg :: Type (ty) => { cx . tcx . sess . source_map () . span_to_snippet (ty . span) . unwrap_or_else (| _ | "_" . into ()) } GenericArg :: Const (c) => cx . tcx . sess . source_map () . span_to_snippet (c . span ()) . unwrap_or_else (| _ | "_" . into ()) , GenericArg :: Infer (_) => String :: from ("_") , }) . collect :: < Vec < _ > > () ; if ! params . is_empty () { return format ! ("<{}>" , params . join (", ")) ; } } String :: new () }
};
}
