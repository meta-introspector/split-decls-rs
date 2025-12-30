// Generated macro for check_for_overlapping_test_paths (function)
macro_rules! Depcratecheck_for_overlapping_test_paths {
() => {
// Module: crate
// Provides: {"check_for_overlapping_test_paths"}
// Dependencies: {}
# [doc = " Checks that test discovery didn't find any tests whose name stem is a prefix"] # [doc = " of some other tests's name."] # [doc = ""] # [doc = " For example, suppose the test suite contains these two test files:"] # [doc = " - `tests/rustdoc/primitive.rs`"] # [doc = " - `tests/rustdoc/primitive/no_std.rs`"] # [doc = ""] # [doc = " The test runner might put the output from those tests in these directories:"] # [doc = " - `$build/test/rustdoc/primitive/`"] # [doc = " - `$build/test/rustdoc/primitive/no_std/`"] # [doc = ""] # [doc = " Because one output path is a subdirectory of the other, the two tests might"] # [doc = " interfere with each other in unwanted ways, especially if the test runner"] # [doc = " decides to delete test output directories to clean them between runs."] # [doc = " To avoid problems, we forbid test names from overlapping in this way."] # [doc = ""] # [doc = " See <https://github.com/rust-lang/rust/pull/109509> for more context."] fn check_for_overlapping_test_paths (found_path_stems : & HashSet < Utf8PathBuf >) { let mut collisions = Vec :: new () ; for path in found_path_stems { for ancestor in path . ancestors () . skip (1) { if found_path_stems . contains (ancestor) { collisions . push ((path , ancestor)) ; } } } if ! collisions . is_empty () { collisions . sort () ; let collisions : String = collisions . into_iter () . map (| (path , check_parent) | format ! ("test {path} clashes with {check_parent}\n")) . collect () ; panic ! ("{collisions}\n\
            Tests cannot have overlapping names. Make sure they use unique prefixes.") ; } }
};
}
