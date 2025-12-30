// Generated macro for usage (function)
macro_rules! Depcrateusage {
() => {
// Module: crate
// Provides: {"usage"}
// Dependencies: {}
fn usage () { println ! ("\
rustc_codegen_gcc build system

Usage: build_system [command] [options]

Options:
        --help    : Displays this help message.

Commands:
        cargo     : Executes a cargo command.
        rustc     : Compiles the program using the GCC compiler.
        clean     : Cleans the build directory, removing all compiled files and artifacts.
        prepare   : Prepares the environment for building, including fetching dependencies and setting up configurations.
        build     : Compiles the project.
        test      : Runs tests for the project.
        info      : Displays information about the build environment and project configuration.
        clone-gcc : Clones the GCC compiler from a specified source.
        fmt       : Runs rustfmt
        fuzz      : Fuzzes `cg_gcc` using rustlantis
        abi-test   : Runs the abi-cafe test suite on the codegen, checking for ABI compatibility with LLVM") ; }
};
}
