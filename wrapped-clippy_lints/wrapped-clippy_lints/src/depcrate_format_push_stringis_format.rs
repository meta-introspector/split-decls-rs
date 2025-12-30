// Generated macro for is_format (function)
macro_rules! Depcrate_format_push_stringis_format {
() => {
// Module: crate::format_push_string
// Provides: {"is_format"}
// Dependencies: {}
fn is_format (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { let e = e . peel_blocks () . peel_borrows () ; if e . span . from_expansion () && let Some (macro_def_id) = e . span . ctxt () . outer_expn_data () . macro_def_id { cx . tcx . get_diagnostic_name (macro_def_id) == Some (sym :: format_macro) } else if let Some (higher :: If { then , r#else , .. }) = higher :: If :: hir (e) { is_format (cx , then) || r#else . is_some_and (| e | is_format (cx , e)) } else { match higher :: IfLetOrMatch :: parse (cx , e) { Some (higher :: IfLetOrMatch :: Match (_ , arms , MatchSource :: Normal)) => { arms . iter () . any (| arm | is_format (cx , arm . body)) } , Some (higher :: IfLetOrMatch :: IfLet (_ , _ , then , r#else , _)) => { is_format (cx , then) || r#else . is_some_and (| e | is_format (cx , e)) } , _ => false , } } }
};
}
