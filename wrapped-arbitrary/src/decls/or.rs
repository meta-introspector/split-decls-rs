macro_rules! or {
    () => {
        # [doc = " Take the minimum of the lower bounds and maximum of the upper bounds in the"] # [doc = " `lhs` and `rhs` size hints."] # [inline] pub fn or (lhs : (usize , Option < usize >) , rhs : (usize , Option < usize >)) -> (usize , Option < usize >) { let lower = std :: cmp :: min (lhs . 0 , rhs . 0) ; let upper = lhs . 1 . and_then (| lhs | rhs . 1 . map (| rhs | std :: cmp :: max (lhs , rhs))) ; (lower , upper) }
    };
}

or!();