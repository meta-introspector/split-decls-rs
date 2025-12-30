// Generated macro for impl_351 (impl)
macro_rules! Depcrate_macrosimpl_351 {
() => {
// Module: crate::macros
// Provides: {"impl_351"}
// Dependencies: {}
impl < 'a > PanicExpn < 'a > { pub fn parse (expr : & 'a Expr < 'a >) -> Option < Self > { let ExprKind :: Call (callee , args) = & expr . kind else { return None ; } ; let ExprKind :: Path (QPath :: Resolved (_ , path)) = & callee . kind else { return None ; } ; let name = path . segments . last () . unwrap () . ident . name ; let [arg , rest @ ..] = args else { return None ; } ; let result = match name { sym :: panic if arg . span . eq_ctxt (expr . span) => Self :: Empty , sym :: panic | sym :: panic_str => Self :: Str (arg) , sym :: panic_display => { let ExprKind :: AddrOf (_ , _ , e) = & arg . kind else { return None ; } ; Self :: Display (e) } , sym :: panic_fmt => Self :: Format (arg) , sym :: assert_failed => { if rest . len () != 3 { return None ; } let msg_arg = & rest [2] ; match msg_arg . kind { ExprKind :: Call (_ , [fmt_arg]) => Self :: Format (fmt_arg) , _ => Self :: Empty , } } , _ => return None , } ; Some (result) } }
};
}
