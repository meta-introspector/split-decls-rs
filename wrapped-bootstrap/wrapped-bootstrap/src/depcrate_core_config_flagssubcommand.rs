// Generated macro for Subcommand (enum)
macro_rules! Depcrate_core_config_flagsSubcommand {
() => {
// Module: crate::core::config::flags
// Provides: {"Subcommand"}
// Dependencies: {}
# [derive (Debug , Clone , clap :: Subcommand)] pub enum Subcommand { # [command (aliases = ["b"] , long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to directories to the crates
        and/or artifacts to compile. For example, for a quick build of a usable
        compiler:
            ./x.py build --stage 1 library/std
        This will build a compiler and standard library from the local source code.
        Once this is done, build/$ARCH/stage1 contains a usable compiler.
        If no arguments are passed then the default artifacts for that stage are
        compiled. For example:
            ./x.py build --stage 0
            ./x.py build ")] # [doc = " Compile either the compiler or libraries"] Build { # [arg (long)] # [doc = " Pass `--timings` to Cargo to get crate build timings"] timings : bool , } , # [command (aliases = ["c"] , long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to directories to the crates
        and/or artifacts to compile. For example:
            ./x.py check library/std
        If no arguments are passed then many artifacts are checked.")] # [doc = " Compile either the compiler or libraries, using cargo check"] Check { # [arg (long)] # [doc = " Check all targets"] all_targets : bool , # [arg (long)] # [doc = " Pass `--timings` to Cargo to get crate build timings"] timings : bool , } , # [doc = " Run Clippy (uses rustup/cargo-installed clippy binary)"] # [command (long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to directories to the crates
        and/or artifacts to run clippy against. For example:
            ./x.py clippy library/core
            ./x.py clippy library/core library/proc_macro")] Clippy { # [arg (long)] fix : bool , # [arg (long , requires = "fix")] allow_dirty : bool , # [arg (long , requires = "fix")] allow_staged : bool , # [doc = " clippy lints to allow"] # [arg (global = true , short = 'A' , action = clap :: ArgAction :: Append , value_name = "LINT")] allow : Vec < String > , # [doc = " clippy lints to deny"] # [arg (global = true , short = 'D' , action = clap :: ArgAction :: Append , value_name = "LINT")] deny : Vec < String > , # [doc = " clippy lints to warn on"] # [arg (global = true , short = 'W' , action = clap :: ArgAction :: Append , value_name = "LINT")] warn : Vec < String > , # [doc = " clippy lints to forbid"] # [arg (global = true , short = 'F' , action = clap :: ArgAction :: Append , value_name = "LINT")] forbid : Vec < String > , } , # [doc = " Run cargo fix"] # [command (long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to directories to the crates
        and/or artifacts to run `cargo fix` against. For example:
            ./x.py fix library/core
            ./x.py fix library/core library/proc_macro")] Fix , # [command (name = "fmt" , long_about = "\n
    Arguments:
        This subcommand optionally accepts a `--check` flag which succeeds if
        formatting is correct and fails if it is not. For example:
            ./x.py fmt
            ./x.py fmt --check")] # [doc = " Run rustfmt"] Format { # [doc = " check formatting instead of applying"] # [arg (long)] check : bool , # [doc = " apply to all appropriate files, not just those that have been modified"] # [arg (long)] all : bool , } , # [command (aliases = ["d"] , long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to directories of documentation
        to build. For example:
            ./x.py doc src/doc/book
            ./x.py doc src/doc/nomicon
            ./x.py doc src/doc/book library/std
            ./x.py doc library/std --json
            ./x.py doc library/std --open
        If no arguments are passed then everything is documented:
            ./x.py doc
            ./x.py doc --stage 1")] # [doc = " Build documentation"] Doc { # [arg (long)] # [doc = " open the docs in a browser"] open : bool , # [arg (long)] # [doc = " render the documentation in JSON format in addition to the usual HTML format"] json : bool , } , # [command (aliases = ["t"] , long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to test directories that
        should be compiled and run. For example:
            ./x.py test tests/ui
            ./x.py test library/std --test-args hash_map
            ./x.py test library/std --stage 0 --no-doc
            ./x.py test tests/ui --bless
            ./x.py test tests/ui --compare-mode next-solver
        Note that `test tests/* --stage N` does NOT depend on `build compiler/rustc --stage N`;
        just like `build library/std --stage N` it tests the compiler produced by the previous
        stage.
        Execute tool tests with a tool name argument:
            ./x.py test tidy
        If no arguments are passed then the complete artifacts for that stage are
        compiled and tested.
            ./x.py test
            ./x.py test --stage 1")] # [doc = " Build and run some test suites"] Test { # [arg (long)] # [doc = " run all tests regardless of failure"] no_fail_fast : bool , # [arg (long , value_name = "ARGS" , allow_hyphen_values (true))] # [doc = " extra arguments to be passed for the test tool being used"] # [doc = " (e.g. libtest, compiletest or rustdoc)"] test_args : Vec < String > , # [doc = " extra options to pass the compiler when running compiletest tests"] # [arg (long , value_name = "ARGS" , allow_hyphen_values (true))] compiletest_rustc_args : Vec < String > , # [arg (long)] # [doc = " do not run doc tests"] no_doc : bool , # [arg (long)] # [doc = " only run doc tests"] doc : bool , # [arg (long)] # [doc = " whether to automatically update stderr/stdout files"] bless : bool , # [arg (long)] # [doc = " comma-separated list of other files types to check (accepts py, py:lint,"] # [doc = " py:fmt, shell, cpp, cpp:fmt, js, js:lint, js:typecheck, spellcheck)"] # [doc = ""] # [doc = " Any argument can be prefixed with \"auto:\" to only run if"] # [doc = " relevant files are modified (eg. \"auto:py\")."] extra_checks : Option < String > , # [arg (long)] # [doc = " rerun tests even if the inputs are unchanged"] force_rerun : bool , # [arg (long)] # [doc = " only run tests that result has been changed"] only_modified : bool , # [arg (long , value_name = "COMPARE MODE")] # [doc = " mode describing what file the actual ui output will be compared to"] compare_mode : Option < String > , # [arg (long , value_name = "check | build | run")] # [doc = " force {check,build,run}-pass tests to this mode."] pass : Option < String > , # [arg (long , value_name = "auto | always | never")] # [doc = " whether to execute run-* tests"] run : Option < String > , # [arg (long)] # [doc = " enable this to generate a Rustfix coverage file, which is saved in"] # [doc = " `/<build_base>/rustfix_missing_coverage.txt`"] rustfix_coverage : bool , # [arg (long)] # [doc = " don't capture stdout/stderr of tests"] no_capture : bool , # [arg (long)] # [doc = " Use a different codegen backend when running tests."] test_codegen_backend : Option < CodegenBackendKind > , } , # [doc = " Build and run some test suites *in Miri*"] Miri { # [arg (long)] # [doc = " run all tests regardless of failure"] no_fail_fast : bool , # [arg (long , value_name = "ARGS" , allow_hyphen_values (true))] # [doc = " extra arguments to be passed for the test tool being used"] # [doc = " (e.g. libtest, compiletest or rustdoc)"] test_args : Vec < String > , # [arg (long)] # [doc = " do not run doc tests"] no_doc : bool , # [arg (long)] # [doc = " only run doc tests"] doc : bool , } , # [doc = " Build and run some benchmarks"] Bench { # [arg (long , allow_hyphen_values (true))] test_args : Vec < String > , } , # [doc = " Clean out build directories"] Clean { # [arg (long)] # [doc = " Clean the entire build directory (not used by default)"] all : bool , # [arg (long , value_name = "N")] # [doc = " Clean a specific stage without touching other artifacts. By default, every stage is cleaned if this option is not used."] stage : Option < u32 > , } , # [doc = " Build distribution artifacts"] Dist , # [doc = " Install distribution artifacts"] Install , # [command (aliases = ["r"] , long_about = "\n
    Arguments:
        This subcommand accepts a number of paths to tools to build and run. For
        example:
            ./x.py run src/tools/bump-stage0
        At least a tool needs to be called.")] # [doc = " Run tools contained in this repository"] Run { # [doc = " arguments for the tool"] # [arg (long , allow_hyphen_values (true))] args : Vec < String > , } , # [doc = " Set up the environment for development"] # [command (long_about = format ! ("\n
x.py setup creates a `bootstrap.toml` which changes the defaults for x.py itself,
as well as setting up a git pre-push hook, VS Code config and toolchain link.
Arguments:
    This subcommand accepts a 'profile' to use for builds. For example:
        ./x.py setup library
    The profile is optional and you will be prompted interactively if it is not given.
    The following profiles are available:
{}
    To only set up the git hook, editor config or toolchain link, you may use
        ./x.py setup hook
        ./x.py setup editor
        ./x.py setup link" , Profile :: all_for_help ("        ") . trim_end ()))] Setup { # [doc = " Either the profile for `bootstrap.toml` or another setup action."] # [doc = " May be omitted to set up interactively"] # [arg (value_name = "<PROFILE>|hook|editor|link")] profile : Option < PathBuf > , } , # [doc = " Vendor dependencies"] Vendor { # [doc = " Additional `Cargo.toml` to sync and vendor"] # [arg (long)] sync : Vec < PathBuf > , # [doc = " Always include version in subdir name"] # [arg (long)] versioned_dirs : bool , } , # [doc = " Perform profiling and benchmarking of the compiler using `rustc-perf`."] Perf (PerfArgs) , }
};
}
