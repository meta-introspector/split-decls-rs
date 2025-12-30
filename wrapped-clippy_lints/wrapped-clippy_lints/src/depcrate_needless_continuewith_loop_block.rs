// Generated macro for with_loop_block (function)
macro_rules! Depcrate_needless_continuewith_loop_block {
() => {
// Module: crate::needless_continue
// Provides: {"with_loop_block"}
// Dependencies: {}
# [doc = " If `expr` is a loop expression (while/while let/for/loop), calls `func` with"] # [doc = " the HIR object representing the loop block of `expr`."] fn with_loop_block < F > (expr : & Expr < '_ > , mut func : F) where F : FnMut (& Block < '_ > , Option < & Label >) , { if let Some (higher :: ForLoop { body , label , .. }) = higher :: ForLoop :: hir (expr) && let ExprKind :: Block (block , _) = & body . kind { func (block , label . as_ref ()) ; return ; } if let Some (higher :: While { body , label , .. }) = higher :: While :: hir (expr) && let ExprKind :: Block (block , _) = & body . kind { func (block , label . as_ref ()) ; return ; } if let Some (higher :: WhileLet { if_then , label , .. }) = higher :: WhileLet :: hir (expr) && let ExprKind :: Block (block , _) = & if_then . kind { func (block , label . as_ref ()) ; return ; } if let ExprKind :: Loop (block , label , LoopSource :: Loop , ..) = expr . kind { func (block , label . as_ref ()) ; } }
};
}
