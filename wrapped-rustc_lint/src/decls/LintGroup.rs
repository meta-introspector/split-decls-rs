macro_rules! deps {
    () => {
        LintAlias!();
    };
}

macro_rules! LintGroup {
    () => {
        deps!();
        struct LintGroup { lint_ids : Vec < LintId > , is_externally_loaded : bool , depr : Option < LintAlias > , }
    };
}

LintGroup!()