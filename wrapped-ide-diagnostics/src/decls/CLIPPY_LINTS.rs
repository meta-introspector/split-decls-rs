macro_rules! deps {
    () => {
        BuiltLint!();
    };
}

macro_rules! CLIPPY_LINTS {
    () => {
        deps!();
        static CLIPPY_LINTS : LazyLock < FxHashMap < & str , BuiltLint > > = LazyLock :: new (| | { build_lints_map (ide_db :: generated :: lints :: CLIPPY_LINTS , CLIPPY_LINT_GROUPS , "clippy::") }) ;
    };
}

CLIPPY_LINTS!();