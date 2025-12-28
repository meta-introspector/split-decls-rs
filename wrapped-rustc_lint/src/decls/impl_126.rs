macro_rules! deps {
    () => {
        LintLevelsBuilder!();
        EarlyContext!();
        LintStore!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a > EarlyContext < 'a > { pub (crate) fn new (sess : & 'a Session , features : & 'a Features , lint_added_lints : bool , lint_store : & 'a LintStore , registered_tools : & 'a RegisteredTools , buffered : LintBuffer ,) -> EarlyContext < 'a > { EarlyContext { builder : LintLevelsBuilder :: new (sess , features , lint_added_lints , lint_store , registered_tools ,) , buffered , } } }
    };
}

impl_126!();