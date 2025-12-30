// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [cfg (not (target_os = "haiku"))] fn test_fallback_on_non_haiku_platforms () { assert ! (super :: get_timezone () . is_none ()) ; } # [test] # [cfg (target_os = "haiku")] fn test_retrieve_time_zone_on_haiku_platforms () { let timezone = super :: get_timezone () . unwrap () ; assert ! (! timezone . is_empty ()) ; } }
};
}
