// Generated macro for VERSION (static)
macro_rules! DepcrateVERSION {
() => {
// Module: crate
// Provides: {"VERSION"}
// Dependencies: {}
static VERSION : std :: sync :: LazyLock < (u32 , bool) > = LazyLock :: new (| | { let output = Command :: new ("rustc") . arg ("-V") . output () . expect ("rustc should run") ; let stdout = std :: str :: from_utf8 (& output . stdout) . expect ("utf8") ; let vers = stdout . split_whitespace () . skip (1) . next () . unwrap () ; let is_nightly = option_env ! ("CARGO_TEST_DISABLE_NIGHTLY") . is_none () && (vers . contains ("-nightly") || vers . contains ("-dev")) ; let minor = vers . split ('.') . skip (1) . next () . unwrap () . parse () . unwrap () ; (minor , is_nightly) }) ;
};
}
