macro_rules! ComparisonResult {
    () => {
        enum ComparisonResult { Improved , Regressed , NonSignificant , }
    };
}

ComparisonResult!();