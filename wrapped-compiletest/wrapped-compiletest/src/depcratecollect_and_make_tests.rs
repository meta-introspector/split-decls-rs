// Generated macro for collect_and_make_tests (function)
macro_rules! Depcratecollect_and_make_tests {
() => {
// Module: crate
// Provides: {"collect_and_make_tests"}
// Dependencies: {}
# [doc = " Creates test structures for every test/revision in the test suite directory."] # [doc = ""] # [doc = " This always inspects _all_ test files in the suite (e.g. all 17k+ ui tests),"] # [doc = " regardless of whether any filters/tests were specified on the command-line,"] # [doc = " because filtering is handled later by code that was copied from libtest."] # [doc = ""] # [doc = " FIXME(Zalathar): Now that we no longer rely on libtest, try to overhaul"] # [doc = " test discovery to take into account the filters/tests specified on the"] # [doc = " command-line, instead of having to enumerate everything."] pub (crate) fn collect_and_make_tests (config : Arc < Config >) -> Vec < CollectedTest > { debug ! ("making tests from {}" , config . src_test_suite_root) ; let common_inputs_stamp = common_inputs_stamp (& config) ; let modified_tests = modified_tests (& config , & config . src_test_suite_root) . unwrap_or_else (| err | { fatal ! ("modified_tests: {}: {err}" , config . src_test_suite_root) ; }) ; let cache = DirectivesCache :: load (& config) ; let cx = TestCollectorCx { config , cache , common_inputs_stamp , modified_tests } ; let collector = collect_tests_from_dir (& cx , & cx . config . src_test_suite_root , Utf8Path :: new ("")) . unwrap_or_else (| reason | { panic ! ("Could not read tests from {}: {reason}" , cx . config . src_test_suite_root) }) ; let TestCollector { tests , found_path_stems , poisoned } = collector ; if poisoned { eprintln ! () ; panic ! ("there are errors in tests") ; } check_for_overlapping_test_paths (& found_path_stems) ; tests }
};
}
