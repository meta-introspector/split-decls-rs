// Generated macro for get_lint_tbls (function)
macro_rules! Depcrate_cargo_lint_groups_priorityget_lint_tbls {
() => {
// Module: crate::cargo::lint_groups_priority
// Provides: {"get_lint_tbls"}
// Dependencies: {}
fn get_lint_tbls < 'a > (tbl : & 'a DeTable < 'a >) -> LintTbls < 'a > { if let Some (lints) = tbl . get ("lints") && let Some (lints) = lints . get_ref () . as_table () { let rust = lints . get ("rust") . and_then (| x | x . get_ref () . as_table ()) ; let clippy = lints . get ("clippy") . and_then (| x | x . get_ref () . as_table ()) ; LintTbls { rust , clippy } } else { LintTbls { rust : None , clippy : None , } } }
};
}
