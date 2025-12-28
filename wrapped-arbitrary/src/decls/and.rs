macro_rules! and {
    () => {
        # [doc = " Take the sum of the `lhs` and `rhs` size hints."] # [inline] pub fn and (lhs : (usize , Option < usize >) , rhs : (usize , Option < usize >)) -> (usize , Option < usize >) { let lower = lhs . 0 + rhs . 0 ; let upper = lhs . 1 . and_then (| lhs | rhs . 1 . map (| rhs | lhs + rhs)) ; (lower , upper) }
    };
}

and!();