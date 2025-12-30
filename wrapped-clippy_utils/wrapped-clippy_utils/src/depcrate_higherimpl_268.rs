// Generated macro for impl_268 (impl)
macro_rules! Depcrate_higherimpl_268 {
() => {
// Module: crate::higher
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'hir > IfLetOrMatch < 'hir > { # [doc = " Parses an `if let` or `match` expression"] pub fn parse (cx : & LateContext < '_ > , expr : & Expr < 'hir >) -> Option < Self > { match expr . kind { ExprKind :: Match (expr , arms , source) => Some (Self :: Match (expr , arms , source)) , _ => IfLet :: hir (cx , expr) . map (| IfLet { let_expr , let_pat , if_then , if_else , let_span , } | { Self :: IfLet (let_expr , let_pat , if_then , if_else , let_span) } ,) , } } pub fn scrutinee (& self) -> & 'hir Expr < 'hir > { match self { Self :: Match (scrutinee , _ , _) | Self :: IfLet (scrutinee , _ , _ , _ , _) => scrutinee , } } }
};
}
