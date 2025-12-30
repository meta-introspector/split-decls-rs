// Generated macro for test_explicit_stage (function)
macro_rules! Depcrate_core_config_teststest_explicit_stage {
() => {
// Module: crate::core::config::tests
// Provides: {"test_explicit_stage"}
// Dependencies: {}
# [test] fn test_explicit_stage () { let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--config=/does/not/exist" . to_owned ()]) , | & _ | { toml :: from_str (r#"
            [build]
            test-stage = 1
        "# ,) } ,) ; assert ! (! config . explicit_stage_from_cli) ; assert ! (config . explicit_stage_from_config) ; assert ! (config . is_explicit_stage ()) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--stage=2" . to_owned () , "--config=/does/not/exist" . to_owned () ,]) , | & _ | toml :: from_str ("") ,) ; assert ! (config . explicit_stage_from_cli) ; assert ! (! config . explicit_stage_from_config) ; assert ! (config . is_explicit_stage ()) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--stage=2" . to_owned () , "--config=/does/not/exist" . to_owned () ,]) , | & _ | { toml :: from_str (r#"
            [build]
            test-stage = 1
        "# ,) } ,) ; assert ! (config . explicit_stage_from_cli) ; assert ! (config . explicit_stage_from_config) ; assert ! (config . is_explicit_stage ()) ; let config = Config :: parse_inner (Flags :: parse (& ["check" . to_owned () , "--config=/does/not/exist" . to_owned ()]) , | & _ | toml :: from_str ("") ,) ; assert ! (! config . explicit_stage_from_cli) ; assert ! (! config . explicit_stage_from_config) ; assert ! (! config . is_explicit_stage ()) ; }
};
}
