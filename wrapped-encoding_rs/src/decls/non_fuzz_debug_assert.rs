macro_rules! non_fuzz_debug_assert {
    () => {
        macro_rules ! non_fuzz_debug_assert { ($ ($ arg : tt) *) => (if ! cfg ! (fuzzing) { debug_assert ! ($ ($ arg) *) ; }) }
    };
}

non_fuzz_debug_assert!()