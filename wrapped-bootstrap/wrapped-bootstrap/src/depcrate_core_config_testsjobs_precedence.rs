// Generated macro for jobs_precedence (function)
macro_rules! Depcrate_core_config_testsjobs_precedence {
() => {
// Module: crate::core::config::tests
// Provides: {"jobs_precedence"}
// Dependencies: {}
# [test] fn jobs_precedence () { let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--config=/does/not/exist" . to_owned () , "--jobs=67890" . to_owned () , "--set=build.jobs=12345" . to_owned () ,]) , | & _ | toml :: from_str ("") ,) ; assert_eq ! (config . jobs , Some (67890)) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--config=/does/not/exist" . to_owned () , "--set=build.jobs=12345" . to_owned () ,]) , | & _ | { toml :: from_str (r#"
            [build]
            jobs = 67890
        "# ,) } ,) ; assert_eq ! (config . jobs , Some (12345)) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--jobs=123" . to_owned () , "--config=/does/not/exist" . to_owned () , "--set=build.jobs=456" . to_owned () ,]) , | & _ | { toml :: from_str (r#"
            [build]
            jobs = 789
        "# ,) } ,) ; assert_eq ! (config . jobs , Some (123)) ; }
};
}
