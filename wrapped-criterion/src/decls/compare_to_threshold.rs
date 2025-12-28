macro_rules! deps {
    () => {
        Estimate!();
        ComparisonResult!();
    };
}

macro_rules! compare_to_threshold {
    () => {
        deps!();
        fn compare_to_threshold (estimate : & Estimate , noise : f64) -> ComparisonResult { let ci = & estimate . confidence_interval ; let lb = ci . lower_bound ; let ub = ci . upper_bound ; if lb < - noise && ub < - noise { ComparisonResult :: Improved } else if lb > noise && ub > noise { ComparisonResult :: Regressed } else { ComparisonResult :: NonSignificant } }
    };
}

compare_to_threshold!();