macro_rules! TINY_LINT_TERMINATOR_LIMIT {
    () => {
        # [doc = " The limit used by `-Z tiny-const-eval-limit`. This smaller limit is useful for internal"] # [doc = " tests not needing to run 30s or more to show some behaviour."] const TINY_LINT_TERMINATOR_LIMIT : usize = 20 ;
    };
}

TINY_LINT_TERMINATOR_LIMIT!();