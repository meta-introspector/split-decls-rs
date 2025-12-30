// Generated macro for is_default_equivalent (function)
macro_rules! Depcrateis_default_equivalent {
() => {
// Module: crate
// Provides: {"is_default_equivalent"}
// Dependencies: {}
# [doc = " Returns true if the expr is equal to `Default::default()` of its type when evaluated."] # [doc = ""] # [doc = " It doesn't cover all cases, like struct literals, but it is a close approximation."] pub fn is_default_equivalent (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { match & e . kind { ExprKind :: Lit (lit) => match lit . node { LitKind :: Bool (false) | LitKind :: Int (Pu128 (0) , _) => true , LitKind :: Str (s , _) => s . is_empty () , _ => false , } , ExprKind :: Tup (items) | ExprKind :: Array (items) => items . iter () . all (| x | is_default_equivalent (cx , x)) , ExprKind :: Repeat (x , len) => { if let ConstArgKind :: Anon (anon_const) = len . kind && let ExprKind :: Lit (const_lit) = cx . tcx . hir_body (anon_const . body) . value . kind && let LitKind :: Int (v , _) = const_lit . node && v <= 32 && is_default_equivalent (cx , x) { true } else { false } } , ExprKind :: Call (repl_func , []) => is_default_equivalent_call (cx , repl_func , Some (e)) , ExprKind :: Call (from_func , [arg]) => is_default_equivalent_from (cx , from_func , arg) , ExprKind :: Path (qpath) => cx . qpath_res (qpath , e . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone) , ExprKind :: AddrOf (rustc_hir :: BorrowKind :: Ref , _ , expr) => matches ! (expr . kind , ExprKind :: Array ([])) , ExprKind :: Block (Block { stmts : [] , expr , .. } , _) => expr . is_some_and (| e | is_default_equivalent (cx , e)) , _ => false , } }
};
}
