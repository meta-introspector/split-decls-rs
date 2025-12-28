macro_rules! deps {
    () => {
        EarlyLintPassObject!();
    };
}

macro_rules! RuntimeCombinedEarlyLintPass {
    () => {
        deps!();
        struct RuntimeCombinedEarlyLintPass < 'a > { passes : & 'a mut [EarlyLintPassObject] , }
    };
}

RuntimeCombinedEarlyLintPass!();