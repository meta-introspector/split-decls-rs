// Generated macro for impl_687 (impl)
macro_rules! Depcrate_cargo_lint_groups_priorityimpl_687 {
() => {
// Module: crate::cargo::lint_groups_priority
// Provides: {"impl_687"}
// Dependencies: {}
impl < 'a > LintConfig < 'a > { fn priority (& self) -> i64 { self . priority . unwrap_or (0) } fn is_implicit (& self) -> bool { self . priority . is_none () } fn parse (value : & 'a Spanned < DeValue < 'a > >) -> Option < Self > { let sp = value . span () ; let (level , priority) = match value . get_ref () { DeValue :: String (level) => (& * * level , None) , DeValue :: Table (tbl) => { let level = tbl . get ("level") ? . get_ref () . as_str () ? ; let priority = if let Some (priority) = tbl . get ("priority") { let priority = priority . get_ref () . as_integer () ? ; Some (i64 :: from_str_radix (priority . as_str () , priority . radix ()) . ok () ?) } else { None } ; (level , priority) } , _ => return None , } ; Some (Self { sp , level , priority }) } }
};
}
