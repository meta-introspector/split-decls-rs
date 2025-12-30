// Generated macro for ProcessedArgs (struct)
macro_rules! DepcrateProcessedArgs {
() => {
// Module: crate
// Provides: {"ProcessedArgs"}
// Dependencies: {}
# [derive (Parser)] # [command (about = "Collect a memory report for examples using dhat-rs.")] struct ProcessedArgs { # [arg (long , value_name = "OS" , help = "Nests the results of the benchmark in a folder per-OS, primarily needed by CI.")] os : Option < String > , # [arg (value_name = "EXAMPLES" , num_args = 0 .., index = 1)] # [arg (help = "The space separated list of examples to run. Leave empty for all examples.")] examples : Vec < String > , }
};
}
