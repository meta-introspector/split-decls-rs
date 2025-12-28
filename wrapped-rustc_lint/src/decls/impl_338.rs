macro_rules! deps {
    () => {
        LintLevelsProvider!();
        TopDown!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl LintLevelsProvider for TopDown { fn current_specs (& self) -> & FxIndexMap < LintId , LevelAndSource > { & self . sets . list [self . cur] . specs } fn insert (& mut self , id : LintId , lvl : LevelAndSource) { self . sets . list [self . cur] . specs . insert (id , lvl) ; } fn get_lint_level (& self , lint : & 'static Lint , sess : & Session) -> LevelAndSource { self . sets . get_lint_level (lint , self . cur , Some (self . current_specs ()) , sess) } fn push_expectation (& mut self , _ : LintExpectationId , _ : LintExpectation) { } }
    };
}

impl_338!()