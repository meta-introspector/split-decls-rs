// Generated macro for LOCALE (static)
macro_rules! Depcrate_output_timeLOCALE {
() => {
// Module: crate::output::time
// Provides: {"LOCALE"}
// Dependencies: {}
static LOCALE : LazyLock < locale :: Time > = LazyLock :: new (| | locale :: Time :: load_user_locale () . unwrap_or_else (| _ | locale :: Time :: english ())) ;
};
}
