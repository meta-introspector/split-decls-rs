macro_rules! SNAPSHOT_TEST_MACROS {
    () => {
        static SNAPSHOT_TEST_MACROS : OnceLock < FxHashMap < & str , Vec < [Symbol ; 2] > > > = OnceLock :: new () ;
    };
}

SNAPSHOT_TEST_MACROS!();