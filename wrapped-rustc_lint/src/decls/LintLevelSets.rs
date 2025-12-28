macro_rules! deps {
    () => {
        LintSet!();
    };
}

macro_rules! LintLevelSets {
    () => {
        deps!();
        # [doc = " Collection of lint levels for the whole crate."] # [doc = " This is used by AST-based lints, which do not"] # [doc = " wait until we have built HIR to be emitted."] # [derive (Debug)] struct LintLevelSets { # [doc = " Linked list of specifications."] list : IndexVec < LintStackIndex , LintSet > , }
    };
}

LintLevelSets!()