// Generated macro for handle_control_flow_keywords (function)
macro_rules! Depcrate_goto_definitionhandle_control_flow_keywords {
() => {
// Module: crate::goto_definition
// Provides: {"handle_control_flow_keywords"}
// Dependencies: {}
fn handle_control_flow_keywords (sema : & Semantics < '_ , RootDatabase > , token : & SyntaxToken ,) -> Option < Vec < NavigationTarget > > { match token . kind () { T ! [fn] | T ! [async] | T ! [try] | T ! [return] => nav_for_exit_points (sema , token) , T ! [loop] | T ! [while] | T ! [break] | T ! [continue] => nav_for_break_points (sema , token) , T ! [for] if token . parent () . and_then (ast :: ForExpr :: cast) . is_some () => { nav_for_break_points (sema , token) } T ! [match] | T ! [=>] | T ! [if] => nav_for_branch_exit_points (sema , token) , _ => None , } }
};
}
