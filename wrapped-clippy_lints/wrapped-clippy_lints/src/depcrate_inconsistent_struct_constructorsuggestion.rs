// Generated macro for suggestion (function)
macro_rules! Depcrate_inconsistent_struct_constructorsuggestion {
() => {
// Module: crate::inconsistent_struct_constructor
// Provides: {"suggestion"}
// Dependencies: {}
fn suggestion < 'tcx > (cx : & LateContext < '_ > , fields : & 'tcx [hir :: ExprField < 'tcx >] , def_order_map : & FxHashMap < Symbol , usize > ,) -> String { let ws = fields . windows (2) . map (| w | { let w0_span = field_with_attrs_span (cx . tcx , & w [0]) ; let w1_span = field_with_attrs_span (cx . tcx , & w [1]) ; let span = w0_span . between (w1_span) ; snippet (cx , span , " ") }) . collect :: < Vec < _ > > () ; let mut fields = fields . to_vec () ; fields . sort_unstable_by_key (| field | def_order_map [& field . ident . name]) ; let field_snippets = fields . iter () . map (| field | snippet (cx , field_with_attrs_span (cx . tcx , field) , "..")) . collect :: < Vec < _ > > () ; assert_eq ! (field_snippets . len () , ws . len () + 1) ; let mut sugg = String :: new () ; for i in 0 .. field_snippets . len () { sugg += & field_snippets [i] ; if i < ws . len () { sugg += & ws [i] ; } } sugg }
};
}
