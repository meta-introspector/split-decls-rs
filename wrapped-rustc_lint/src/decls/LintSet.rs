macro_rules! LintSet {
    () => {
        # [doc = " Specifications found at this position in the stack. This map only represents the lints"] # [doc = " found for one set of attributes (like `shallow_lint_levels_on` does)."] # [doc = ""] # [doc = " We store the level specifications as a linked list."] # [doc = " Each `LintSet` represents a set of attributes on the same AST node."] # [doc = " The `parent` forms a linked list that matches the AST tree."] # [doc = " This way, walking the linked list is equivalent to walking the AST bottom-up"] # [doc = " to find the specifications for a given lint."] # [derive (Debug)] struct LintSet { specs : FxIndexMap < LintId , LevelAndSource > , parent : LintStackIndex , }
    };
}

LintSet!();