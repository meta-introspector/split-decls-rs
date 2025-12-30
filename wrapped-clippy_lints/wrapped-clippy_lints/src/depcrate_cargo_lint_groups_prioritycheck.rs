// Generated macro for check (function)
macro_rules! Depcrate_cargo_lint_groups_prioritycheck {
() => {
// Module: crate::cargo::lint_groups_priority
// Provides: {"check"}
// Dependencies: {}
pub fn check (cx : & LateContext < '_ >) { if let Ok (file) = cx . tcx . sess . source_map () . load_file (Path :: new ("Cargo.toml")) && let Some (src) = file . src . as_deref () && let Ok (cargo_toml) = DeTable :: parse (src) { let mut rustc_groups = FxHashSet :: default () ; let mut clippy_groups = FxHashSet :: default () ; for (group , ..) in unerased_lint_store (cx . tcx . sess) . get_lint_groups () { match group . split_once ("::") { None => { rustc_groups . insert (group) ; } , Some (("clippy" , group)) => { clippy_groups . insert (group) ; } , _ => { } , } } let lints = get_lint_tbls (cargo_toml . get_ref ()) ; if let Some (lints) = lints . rust { check_table (cx , lints , & rustc_groups , & file) ; } if let Some (lints) = lints . clippy { check_table (cx , lints , & clippy_groups , & file) ; } if let Some (tbl) = cargo_toml . get_ref () . get ("workspace") && let Some (tbl) = tbl . get_ref () . as_table () { let lints = get_lint_tbls (tbl) ; if let Some (lints) = lints . rust { check_table (cx , lints , & rustc_groups , & file) ; } if let Some (lints) = lints . clippy { check_table (cx , lints , & clippy_groups , & file) ; } } } }
};
}
