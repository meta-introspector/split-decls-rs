// Generated macro for analyze_binary_dependencies (function)
macro_rules! Depcrateanalyze_binary_dependencies {
() => {
// Module: crate
// Provides: {"analyze_binary_dependencies"}
// Dependencies: {}
fn analyze_binary_dependencies (bin_name : & str) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("🔍 Analyzing dependencies for binary: {}" , bin_name) ; let bin_path = find_binary_in_output2 (bin_name) ? ; println ! ("📍 Found binary at: {}" , bin_path . display ()) ; let content = fs :: read_to_string (& bin_path) ? ; let ast : SynFile = syn :: parse_file (& content) ? ; let mut visitor = TokenVisitor { uses : HashSet :: new () } ; visitor . visit_file (& ast) ; println ! ("🔧 Binary uses {} unique tokens" , visitor . uses . len ()) ; let mut resolved_deps = HashMap :: new () ; scan_output2_for_tokens (& visitor . uses , & mut resolved_deps) ? ; println ! ("✅ Resolved {} dependencies in output2:" , resolved_deps . len ()) ; for (token , paths) in & resolved_deps { println ! ("  {} -> {} files" , token , paths . len ()) ; for path in paths . iter () . take (3) { println ! ("    {}" , path . display ()) ; } if paths . len () > 3 { println ! ("    ... and {} more" , paths . len () - 3) ; } } generate_dependency_graph (bin_name , & resolved_deps) ? ; Ok (()) }
};
}
