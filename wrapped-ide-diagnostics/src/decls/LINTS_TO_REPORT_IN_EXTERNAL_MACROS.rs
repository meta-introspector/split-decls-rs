macro_rules! LINTS_TO_REPORT_IN_EXTERNAL_MACROS {
    () => {
        static LINTS_TO_REPORT_IN_EXTERNAL_MACROS : LazyLock < FxHashSet < & str > > = LazyLock :: new (| | FxHashSet :: from_iter ([])) ;
    };
}

LINTS_TO_REPORT_IN_EXTERNAL_MACROS!()