// Generated macro for RUSTC_LINTS (static)
macro_rules! DepcrateRUSTC_LINTS {
() => {
// Module: crate
// Provides: {"RUSTC_LINTS"}
// Dependencies: {}
static RUSTC_LINTS : LazyLock < FxHashMap < & str , BuiltLint > > = LazyLock :: new (| | build_lints_map (DEFAULT_LINTS , DEFAULT_LINT_GROUPS , "")) ;
};
}
