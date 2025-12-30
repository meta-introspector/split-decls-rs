// Generated macro for impl_1271 (impl)
macro_rules! Depcrate_copiesimpl_1271 {
() => {
// Module: crate::copies
// Provides: {"impl_1271"}
// Dependencies: {}
impl BlockEq { fn start_span (& self , b : & Block < '_ > , sm : & SourceMap) -> Option < Span > { match & b . stmts [.. self . start_end_eq] { [first , .. , last] => Some (sm . stmt_span (first . span , b . span) . to (sm . stmt_span (last . span , b . span))) , [s] => Some (sm . stmt_span (s . span , b . span)) , [] => None , } } fn end_span (& self , b : & Block < '_ > , sm : & SourceMap) -> Option < Span > { match (& b . stmts [b . stmts . len () - self . end_begin_eq ? ..] , b . expr) { ([first , .. , last] , None) => Some (sm . stmt_span (first . span , b . span) . to (sm . stmt_span (last . span , b . span))) , ([first , ..] , Some (last)) => Some (sm . stmt_span (first . span , b . span) . to (sm . stmt_span (last . span , b . span))) , ([s] , None) => Some (sm . stmt_span (s . span , b . span)) , ([] , Some (e)) => Some (walk_chain (e . span , b . span . ctxt ())) , ([] , None) => None , } } }
};
}
