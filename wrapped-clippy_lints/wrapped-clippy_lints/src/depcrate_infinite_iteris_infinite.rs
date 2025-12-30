// Generated macro for is_infinite (function)
macro_rules! Depcrate_infinite_iteris_infinite {
() => {
// Module: crate::infinite_iter
// Provides: {"is_infinite"}
// Dependencies: {}
fn is_infinite (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Finiteness { match expr . kind { ExprKind :: MethodCall (method , receiver , args , _) => { for & (name , len , heuristic , cap) in & HEURISTICS { if method . ident . name == name && args . len () == len { return (match heuristic { Always => Infinite , First => is_infinite (cx , receiver) , Any => is_infinite (cx , receiver) . or (is_infinite (cx , & args [0])) , All => is_infinite (cx , receiver) . and (is_infinite (cx , & args [0])) , }) . and (cap) ; } } if method . ident . name == sym :: flat_map && args . len () == 1 && let ExprKind :: Closure (& Closure { body , .. }) = args [0] . kind { let body = cx . tcx . hir_body (body) ; return is_infinite (cx , body . value) ; } Finite } , ExprKind :: Block (block , _) => block . expr . as_ref () . map_or (Finite , | e | is_infinite (cx , e)) , ExprKind :: AddrOf (BorrowKind :: Ref , _ , e) => is_infinite (cx , e) , ExprKind :: Call (path , _) => { if let ExprKind :: Path (ref qpath) = path . kind { cx . qpath_res (qpath , path . hir_id) . opt_def_id () . is_some_and (| id | cx . tcx . is_diagnostic_item (sym :: iter_repeat , id)) . into () } else { Finite } } , ExprKind :: Struct (..) => higher :: Range :: hir (cx , expr) . is_some_and (| r | r . end . is_none ()) . into () , _ => Finite , } }
};
}
