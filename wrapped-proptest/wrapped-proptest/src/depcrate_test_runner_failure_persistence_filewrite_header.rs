// Generated macro for write_header (function)
macro_rules! Depcrate_test_runner_failure_persistence_filewrite_header {
() => {
// Module: crate::test_runner::failure_persistence::file
// Provides: {"write_header"}
// Dependencies: {}
fn write_header (buf : & mut Vec < u8 >) -> io :: Result < () > { writeln ! (buf , "\
# Seeds for failure cases proptest has generated in the past. It is
# automatically read and these particular cases re-run before any
# novel cases are generated.
#
# It is recommended to check this file in to source control so that
# everyone who runs the test benefits from these saved cases.") }
};
}
