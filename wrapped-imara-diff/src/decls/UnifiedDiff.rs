macro_rules! deps {
    () => {
        UnifiedDiffConfig!();
        Token!();
        Diff!();
        UnifiedDiffPrinter!();
    };
}

macro_rules! UnifiedDiff {
    () => {
        deps!();
        pub struct UnifiedDiff < 'a , P : UnifiedDiffPrinter > { printer : & 'a P , diff : & 'a Diff , config : UnifiedDiffConfig , before : & 'a [Token] , after : & 'a [Token] , }
    };
}

UnifiedDiff!()