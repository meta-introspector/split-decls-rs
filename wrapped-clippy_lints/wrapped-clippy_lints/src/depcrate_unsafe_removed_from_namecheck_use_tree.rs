// Generated macro for check_use_tree (function)
macro_rules! Depcrate_unsafe_removed_from_namecheck_use_tree {
() => {
// Module: crate::unsafe_removed_from_name
// Provides: {"check_use_tree"}
// Dependencies: {}
fn check_use_tree (use_tree : & UseTree , cx : & EarlyContext < '_ > , span : Span) { match use_tree . kind { UseTreeKind :: Simple (Some (new_name)) => { let old_name = use_tree . prefix . segments . last () . expect ("use paths cannot be empty") . ident ; unsafe_to_safe_check (old_name , new_name , cx , span) ; } , UseTreeKind :: Simple (None) | UseTreeKind :: Glob => { } , UseTreeKind :: Nested { ref items , .. } => { for (use_tree , _) in items { check_use_tree (use_tree , cx , span) ; } } , } }
};
}
