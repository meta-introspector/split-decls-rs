// Generated macro for IfBlockType (enum)
macro_rules! Depcrate_question_markIfBlockType {
() => {
// Module: crate::question_mark
// Provides: {"IfBlockType"}
// Dependencies: {}
enum IfBlockType < 'hir > { # [doc = " An `if x.is_xxx() { a } else { b } ` expression."] # [doc = ""] # [doc = " Contains: `caller (x), caller_type, call_sym (is_xxx), if_then (a), if_else (b)`"] IfIs (& 'hir Expr < 'hir > , Ty < 'hir > , Symbol , & 'hir Expr < 'hir >) , # [doc = " An `if let Xxx(a) = b { c } else { d }` expression."] # [doc = ""] # [doc = " Contains: `let_pat_qpath (Xxx), let_pat_type, let_pat_sym (a), let_expr (b), if_then (c),"] # [doc = " if_else (d)`"] IfLet (Res , Ty < 'hir > , Symbol , & 'hir Expr < 'hir > , & 'hir Expr < 'hir > , Option < & 'hir Expr < 'hir > > ,) , }
};
}
