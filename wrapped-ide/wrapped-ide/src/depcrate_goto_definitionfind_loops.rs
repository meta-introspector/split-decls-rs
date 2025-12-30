// Generated macro for find_loops (function)
macro_rules! Depcrate_goto_definitionfind_loops {
() => {
// Module: crate::goto_definition
// Provides: {"find_loops"}
// Dependencies: {}
pub (crate) fn find_loops (sema : & Semantics < '_ , RootDatabase > , token : & SyntaxToken ,) -> Option < Vec < ast :: Expr > > { let parent = token . parent () ? ; let lbl = match_ast ! { match parent { ast :: BreakExpr (break_) => break_ . lifetime () , ast :: ContinueExpr (continue_) => continue_ . lifetime () , _ => None , } } ; let label_matches = | it : Option < ast :: Label > | match (lbl . as_ref () , it . and_then (| it | it . lifetime ())) { (Some (lbl) , Some (it)) => lbl . text () == it . text () , (None , _) => true , (Some (_) , None) => false , } ; let find_ancestors = | token : SyntaxToken | { for anc in sema . token_ancestors_with_macros (token) . filter_map (ast :: Expr :: cast) { let node = match & anc { ast :: Expr :: LoopExpr (loop_) if label_matches (loop_ . label ()) => anc , ast :: Expr :: WhileExpr (while_) if label_matches (while_ . label ()) => anc , ast :: Expr :: ForExpr (for_) if label_matches (for_ . label ()) => anc , ast :: Expr :: BlockExpr (blk) if blk . label () . is_some () && label_matches (blk . label ()) => { anc } _ => continue , } ; return Some (node) ; } None } ; sema . descend_into_macros (token . clone ()) . into_iter () . filter_map (find_ancestors) . collect_vec () . into () }
};
}
