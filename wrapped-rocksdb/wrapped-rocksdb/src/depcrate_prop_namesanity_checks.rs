// Generated macro for sanity_checks (function)
macro_rules! Depcrate_prop_namesanity_checks {
() => {
// Module: crate::prop_name
// Provides: {"sanity_checks"}
// Dependencies: {}
# [test] fn sanity_checks () { let want = "rocksdb.cfstats-no-file-histogram" ; assert_eq ! (want , crate :: properties :: CFSTATS_NO_FILE_HISTOGRAM) ; let want = "rocksdb.num-files-at-level5" ; assert_eq ! (want , &* crate :: properties :: num_files_at_level (5)) ; }
};
}
