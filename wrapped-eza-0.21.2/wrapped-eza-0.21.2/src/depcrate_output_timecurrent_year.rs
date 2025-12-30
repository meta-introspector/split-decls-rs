// Generated macro for CURRENT_YEAR (static)
macro_rules! Depcrate_output_timeCURRENT_YEAR {
() => {
// Module: crate::output::time
// Provides: {"CURRENT_YEAR"}
// Dependencies: {}
static CURRENT_YEAR : LazyLock < i32 > = LazyLock :: new (| | Local :: now () . year ()) ;
};
}
