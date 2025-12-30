// Generated macro for nav_for_break_points (function)
macro_rules! Depcrate_goto_definitionnav_for_break_points {
() => {
// Module: crate::goto_definition
// Provides: {"nav_for_break_points"}
// Dependencies: {}
fn nav_for_break_points (sema : & Semantics < '_ , RootDatabase > , token : & SyntaxToken ,) -> Option < Vec < NavigationTarget > > { let db = sema . db ; let navs = find_loops (sema , token) ? . into_iter () . filter_map (| expr | { let file_id = sema . hir_file_for (expr . syntax ()) ; let expr_in_file = InFile :: new (file_id , expr . clone ()) ; let focus_range = match expr { ast :: Expr :: LoopExpr (loop_) => loop_ . loop_token () ? . text_range () , ast :: Expr :: WhileExpr (while_) => while_ . while_token () ? . text_range () , ast :: Expr :: ForExpr (for_) => for_ . for_token () ? . text_range () , ast :: Expr :: BlockExpr (blk) => blk . label () . unwrap () . syntax () . text_range () , _ => return None , } ; let nav = expr_to_nav (db , expr_in_file , Some (focus_range)) ; Some (nav) }) . flatten () . collect_vec () ; Some (navs) }
};
}
