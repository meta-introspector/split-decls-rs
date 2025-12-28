macro_rules! LintLevelsProvider {
    () => {
        pub trait LintLevelsProvider { fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > ; fn insert (& mut self , id : LintId , lvl : LevelAndSource) ; fn get_lint_level (& self , lint : & 'static Lint , sess : & Session) -> LevelAndSource ; fn push_expectation (& mut self , id : LintExpectationId , expectation : LintExpectation) ; }
    };
}

LintLevelsProvider!()