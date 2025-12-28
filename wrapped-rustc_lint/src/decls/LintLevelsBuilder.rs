macro_rules! deps {
    () => {
        LintStore!();
    };
}

macro_rules! LintLevelsBuilder {
    () => {
        deps!();
        pub struct LintLevelsBuilder < 's , P > { sess : & 's Session , features : & 's Features , provider : P , lint_added_lints : bool , store : & 's LintStore , registered_tools : & 's RegisteredTools , }
    };
}

LintLevelsBuilder!();