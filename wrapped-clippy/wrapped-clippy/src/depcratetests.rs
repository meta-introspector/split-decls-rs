// Generated macro for tests (module)
macro_rules! Depcratetests {
() => {
// Module: crate
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: ClippyCmd ; # [test] fn fix () { let args = "cargo clippy --fix" . split_whitespace () . map (ToString :: to_string) ; let cmd = ClippyCmd :: new (args) ; assert_eq ! ("fix" , cmd . cargo_subcommand) ; assert ! (! cmd . args . iter () . any (| arg | arg . ends_with ("unstable-options"))) ; } # [test] fn fix_implies_no_deps () { let args = "cargo clippy --fix" . split_whitespace () . map (ToString :: to_string) ; let cmd = ClippyCmd :: new (args) ; assert ! (cmd . clippy_args . iter () . any (| arg | arg == "--no-deps")) ; } # [test] fn no_deps_not_duplicated_with_fix () { let args = "cargo clippy --fix -- --no-deps" . split_whitespace () . map (ToString :: to_string) ; let cmd = ClippyCmd :: new (args) ; assert_eq ! (cmd . clippy_args . iter () . filter (| arg | * arg == "--no-deps") . count () , 1) ; } # [test] fn check () { let args = "cargo clippy" . split_whitespace () . map (ToString :: to_string) ; let cmd = ClippyCmd :: new (args) ; assert_eq ! ("check" , cmd . cargo_subcommand) ; } }
};
}
