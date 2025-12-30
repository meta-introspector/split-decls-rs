// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () , Box < dyn std :: error :: Error > > { let cli = Cli :: parse () ; match cli . command { Commands :: ListBins => { scan_output2_binaries () ? ; } , Commands :: Run { bin , args } => { run_binary (& bin , args) ? ; } , Commands :: AnalyzeDeps { bin } => { analyze_binary_dependencies (& bin) ? ; } , Commands :: RecursiveDeps { bin , depth } => { recursive_dependency_analysis (& bin , depth) ? ; } , Commands :: PrintGraph { bin , depth } => { print_dependency_graph (& bin , depth) ? ; } , Commands :: TestEval { bin } => { test_evaluation (& bin) ? ; } , Commands :: Bootstrap => { println ! ("🚀 Incremental Bootstrap Generator") ; fs :: create_dir_all ("../bootstrap3-incremental") ? ; let mut bootstrap = IncrementalBootstrap :: new () ; bootstrap . scan_output2 () ? ; bootstrap . generate_incremental_modules () ? ; println ! ("✨ Incremental bootstrap generation complete!") ; } } Ok (()) }
};
}
