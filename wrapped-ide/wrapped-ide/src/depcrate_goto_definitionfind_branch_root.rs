// Generated macro for find_branch_root (function)
macro_rules! Depcrate_goto_definitionfind_branch_root {
() => {
// Module: crate::goto_definition
// Provides: {"find_branch_root"}
// Dependencies: {}
pub (crate) fn find_branch_root (sema : & Semantics < '_ , RootDatabase > , token : & SyntaxToken ,) -> Vec < SyntaxNode > { let find_nodes = | node_filter : fn (SyntaxNode) -> Option < SyntaxNode > | { sema . descend_into_macros (token . clone ()) . into_iter () . filter_map (| token | node_filter (token . parent () ?)) . collect_vec () } ; match token . kind () { T ! [match] => find_nodes (| node | Some (ast :: MatchExpr :: cast (node) ? . syntax () . clone ())) , T ! [=>] => find_nodes (| node | Some (ast :: MatchArm :: cast (node) ? . syntax () . clone ())) , T ! [if] => find_nodes (| node | { let if_expr = ast :: IfExpr :: cast (node) ? ; let root_if = iter :: successors (Some (if_expr . clone ()) , | if_expr | { let parent_if = if_expr . syntax () . parent () . and_then (ast :: IfExpr :: cast) ? ; let ast :: ElseBranch :: IfExpr (else_branch) = parent_if . else_branch () ? else { return None ; } ; (else_branch . syntax () == if_expr . syntax ()) . then_some (parent_if) }) . last () ? ; Some (root_if . syntax () . clone ()) }) , _ => vec ! [] , } }
};
}
