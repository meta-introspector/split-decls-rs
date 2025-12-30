// Generated macro for impl_614 (impl)
macro_rules! Depcrate_levelsimpl_614 {
() => {
// Module: crate::levels
// Provides: {"impl_614"}
// Dependencies: {}
impl LintLevelSets { fn new () -> Self { LintLevelSets { list : IndexVec :: new () } } fn get_lint_level (& self , lint : & 'static Lint , idx : LintStackIndex , aux : Option < & FxIndexMap < LintId , LevelAndSource > > , sess : & Session ,) -> LevelAndSource { let lint = LintId :: of (lint) ; let (level , mut src) = self . raw_lint_id_level (lint , idx , aux) ; let (level , lint_id) = reveal_actual_level (level , & mut src , sess , lint , | id | { self . raw_lint_id_level (id , idx , aux) }) ; LevelAndSource { level , lint_id , src } } fn raw_lint_id_level (& self , id : LintId , mut idx : LintStackIndex , aux : Option < & FxIndexMap < LintId , LevelAndSource > > ,) -> (Option < (Level , Option < LintExpectationId >) > , LintLevelSource) { if let Some (specs) = aux && let Some (& LevelAndSource { level , lint_id , src }) = specs . get (& id) { return (Some ((level , lint_id)) , src) ; } loop { let LintSet { ref specs , parent } = self . list [idx] ; if let Some (& LevelAndSource { level , lint_id , src }) = specs . get (& id) { return (Some ((level , lint_id)) , src) ; } if idx == COMMAND_LINE { return (None , LintLevelSource :: Default) ; } idx = parent ; } } }
};
}
