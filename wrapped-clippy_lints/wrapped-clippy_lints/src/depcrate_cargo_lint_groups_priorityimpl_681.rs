// Generated macro for impl_681 (impl)
macro_rules! Depcrate_cargo_lint_groups_priorityimpl_681 {
() => {
// Module: crate::cargo::lint_groups_priority
// Provides: {"impl_681"}
// Dependencies: {}
impl LintConfig { fn level (& self) -> & str { match self { LintConfig :: Level (level) => level , LintConfig :: Table (table) => & table . level , } } fn priority (& self) -> i64 { match self { LintConfig :: Level (_) => 0 , LintConfig :: Table (table) => table . priority . unwrap_or (0) , } } fn is_implicit (& self) -> bool { if let LintConfig :: Table (table) = self { table . priority . is_none () } else { true } } }
};
}
