macro_rules! deps {
    () => {
        UnifiedDiffPrinter!();
        Diff!();
        Token!();
        UnifiedDiffConfig!();
    };
}

macro_rules! UnifiedDiff {
    () => {
        deps!();
        pub struct UnifiedDiff < 'a , P : UnifiedDiffPrinter > { printer : & 'a P , diff : & 'a Diff , config : UnifiedDiffConfig , before : & 'a [Token] , after : & 'a [Token] , }
    };
}

UnifiedDiff!();