// Generated macro for CARGO_MIRI_HELP (const)
macro_rules! Depcrate_phasesCARGO_MIRI_HELP {
() => {
// Module: crate::phases
// Provides: {"CARGO_MIRI_HELP"}
// Dependencies: {}
const CARGO_MIRI_HELP : & str = r"Runs binary crates and tests in Miri

Usage:
    cargo miri [subcommand] [<cargo options>...] [--] [<program/test suite options>...]

Subcommands:
    run, r                   Run binaries
    test, t                  Run tests
    nextest                  Run tests with nextest (requires cargo-nextest installed)
    setup                    Only perform automatic setup, but without asking questions (for getting a proper libstd)
    clean                    Clean the Miri cache & target directory

The cargo options are exactly the same as for `cargo run` and `cargo test`, respectively.
Furthermore, the following environment variables are recognized for `run` and `test`:

    MIRIFLAGS                Extra flags to pass to the Miri driver. Use this to pass `-Zmiri-...` flags.

Examples:
    cargo miri run
    cargo miri test -- test-suite-filter

    cargo miri setup --print-sysroot
        This will print the path to the generated sysroot (and nothing else) on stdout.
        stderr will still contain progress information about how the build is doing.

" ;
};
}
