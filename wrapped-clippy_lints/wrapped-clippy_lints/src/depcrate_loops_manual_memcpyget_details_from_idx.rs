// Generated macro for get_details_from_idx (function)
macro_rules! Depcrate_loops_manual_memcpyget_details_from_idx {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"get_details_from_idx"}
// Dependencies: {}
fn get_details_from_idx < 'tcx > (cx : & LateContext < 'tcx > , idx : & Expr < '_ > , starts : & [Start < 'tcx >] ,) -> Option < (StartKind < 'tcx > , Offset) > { fn get_start < 'tcx > (e : & Expr < '_ > , starts : & [Start < 'tcx >]) -> Option < StartKind < 'tcx > > { let id = e . res_local_id () ? ; starts . iter () . find (| start | start . id == id) . map (| start | start . kind) } fn get_offset < 'tcx > (cx : & LateContext < 'tcx > , e : & Expr < '_ > , starts : & [Start < 'tcx >]) -> Option < Sugg < 'static > > { match & e . kind { ExprKind :: Lit (l) => match l . node { ast :: LitKind :: Int (x , _ty) => Some (Sugg :: NonParen (x . to_string () . into ())) , _ => None , } , ExprKind :: Path (..) if get_start (e , starts) . is_none () => Some (Sugg :: hir (cx , e , "???")) , _ => None , } } match idx . kind { ExprKind :: Binary (op , lhs , rhs) => match op . node { BinOpKind :: Add => { let offset_opt = get_start (lhs , starts) . and_then (| s | get_offset (cx , rhs , starts) . map (| o | (s , o))) . or_else (| | get_start (rhs , starts) . and_then (| s | get_offset (cx , lhs , starts) . map (| o | (s , o)))) ; offset_opt . map (| (s , o) | (s , Offset :: positive (o))) } , BinOpKind :: Sub => { get_start (lhs , starts) . and_then (| s | get_offset (cx , rhs , starts) . map (| o | (s , Offset :: negative (o)))) } , _ => None , } , ExprKind :: Path (..) => get_start (idx , starts) . map (| s | (s , Offset :: empty ())) , _ => None , } }
};
}
