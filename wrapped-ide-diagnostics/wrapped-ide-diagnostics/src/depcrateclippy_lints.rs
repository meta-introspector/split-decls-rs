// Generated macro for CLIPPY_LINTS (static)
macro_rules! DepcrateCLIPPY_LINTS {
() => {
// Module: crate
// Provides: {"CLIPPY_LINTS"}
// Dependencies: {}
static CLIPPY_LINTS : LazyLock < FxHashMap < & str , BuiltLint > > = LazyLock :: new (| | { build_lints_map (ide_db :: generated :: lints :: CLIPPY_LINTS , CLIPPY_LINT_GROUPS , "clippy::") }) ;
};
}
