// Generated macro for emit_lint (function)
macro_rules! Depcrate_methods_manual_is_variant_andemit_lint {
() => {
// Module: crate::methods::manual_is_variant_and
// Provides: {"emit_lint"}
// Dependencies: {}
fn emit_lint < 'tcx > (cx : & LateContext < 'tcx > , span : Span , op : Op , flavor : Flavor , in_some_or_ok : bool , map_func : MapFunc < 'tcx > , recv : & Expr < '_ > ,) { let mut app = Applicability :: MachineApplicable ; let recv = snippet_with_applicability (cx , recv . span , "_" , & mut app) ; let (invert_expr , method , invert_body) = match (flavor , op) { (Flavor :: Option , Op :: Eq) => (false , "is_some_and" , ! in_some_or_ok) , (Flavor :: Option , Op :: Ne) => (false , "is_none_or" , in_some_or_ok) , (Flavor :: Result , Op :: Eq) => (false , "is_ok_and" , ! in_some_or_ok) , (Flavor :: Result , Op :: Ne) => (true , "is_ok_and" , ! in_some_or_ok) , } ; span_lint_and_sugg (cx , MANUAL_IS_VARIANT_AND , span , format ! ("called `.map() {op} {pos}()`" , pos = flavor . positive () ,) , "use" , format ! ("{inversion}{recv}.{method}({body})" , inversion = if invert_expr { "!" } else { "" } , body = map_func . sugg (cx , invert_body , & mut app) ,) , app ,) ; }
};
}
