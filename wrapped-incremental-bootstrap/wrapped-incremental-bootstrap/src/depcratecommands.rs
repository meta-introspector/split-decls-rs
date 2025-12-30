// Generated macro for Commands (enum)
macro_rules! DepcrateCommands {
() => {
// Module: crate
// Provides: {"Commands"}
// Dependencies: {}
# [derive (Subcommand , Debug)] enum Commands { # [doc = " List available binaries"] # [command (name = "list-bins")] ListBins , # [doc = " Run a specific binary"] # [command (name = "run")] Run { # [doc = " Binary name to run"] # [arg (long)] bin : String , # [doc = " Arguments to pass to the binary"] args : Vec < String > , } , # [doc = " Analyze dependencies for a specific binary"] # [command (name = "analyze-deps")] AnalyzeDeps { # [doc = " Binary name to analyze"] # [arg (long)] bin : String , } , # [doc = " Recursive dependency analysis with caching"] # [command (name = "recursive-deps")] RecursiveDeps { # [doc = " Binary name to analyze"] # [arg (long)] bin : String , # [doc = " Maximum recursion depth"] # [arg (long , default_value = "3")] depth : usize , } , # [doc = " Print dependency graph for a binary"] # [command (name = "print-graph")] PrintGraph { # [doc = " Binary name to analyze"] # [arg (long)] bin : String , # [doc = " Maximum recursion depth"] # [arg (long , default_value = "2")] depth : usize , } , # [doc = " Test compilation of generated evaluation"] # [command (name = "test-eval")] TestEval { # [doc = " Binary name to test"] # [arg (long)] bin : String , } , # [doc = " Run incremental bootstrap (default)"] # [command (name = "bootstrap")] Bootstrap , }
};
}
