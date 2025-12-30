// Generated macro for impl_621 (impl)
macro_rules! Depcrate_levelsimpl_621 {
() => {
// Module: crate::levels
// Provides: {"impl_621"}
// Dependencies: {}
impl LintLevelsProvider for LintLevelQueryMap < '_ > { fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > { self . specs . specs . get (& self . cur . local_id) . unwrap_or (& self . empty) } fn insert (& mut self , id : LintId , lvl : LevelAndSource) { self . specs . specs . get_mut_or_insert_default (self . cur . local_id) . insert (id , lvl) ; } fn get_lint_level (& self , lint : & 'static Lint , _ : & Session) -> LevelAndSource { self . specs . lint_level_id_at_node (self . tcx , LintId :: of (lint) , self . cur) } fn push_expectation (& mut self , id : LintExpectationId , expectation : LintExpectation) { self . specs . expectations . push ((id , expectation)) } }
};
}
