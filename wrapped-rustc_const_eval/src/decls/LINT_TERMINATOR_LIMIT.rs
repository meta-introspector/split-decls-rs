macro_rules! LINT_TERMINATOR_LIMIT {
    () => {
        # [doc = " When hitting this many interpreted terminators we emit a deny by default lint"] # [doc = " that notfies the user that their constant takes a long time to evaluate. If that's"] # [doc = " what they intended, they can just allow the lint."] const LINT_TERMINATOR_LIMIT : usize = 2_000_000 ;
    };
}

LINT_TERMINATOR_LIMIT!();