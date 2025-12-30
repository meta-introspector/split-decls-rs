// Generated macro for LINTS_TO_REPORT_IN_EXTERNAL_MACROS (static)
macro_rules! DepcrateLINTS_TO_REPORT_IN_EXTERNAL_MACROS {
() => {
// Module: crate
// Provides: {"LINTS_TO_REPORT_IN_EXTERNAL_MACROS"}
// Dependencies: {}
static LINTS_TO_REPORT_IN_EXTERNAL_MACROS : LazyLock < FxHashSet < & str > > = LazyLock :: new (| | FxHashSet :: from_iter ([])) ;
};
}
