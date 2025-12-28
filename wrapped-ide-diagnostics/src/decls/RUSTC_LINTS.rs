macro_rules! deps {
    () => {
        BuiltLint!();
    };
}

macro_rules! RUSTC_LINTS {
    () => {
        deps!();
        static RUSTC_LINTS : LazyLock < FxHashMap < & str , BuiltLint > > = LazyLock :: new (| | build_lints_map (DEFAULT_LINTS , DEFAULT_LINT_GROUPS , "")) ;
    };
}

RUSTC_LINTS!();