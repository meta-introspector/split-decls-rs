// Generated macro for generate_dependency_graph (function)
macro_rules! Depcrategenerate_dependency_graph {
() => {
// Module: crate
// Provides: {"generate_dependency_graph"}
// Dependencies: {}
fn generate_dependency_graph (bin_name : & str , deps : & HashMap < String , Vec < std :: path :: PathBuf > >) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("📊 Generating dependency graph for {}" , bin_name) ; let mut graph_content = format ! ("// Dependency graph for {}\n" , bin_name) ; graph_content . push_str ("// Generated include statements:\n\n") ; let mut included_files = HashSet :: new () ; for (token , paths) in deps { graph_content . push_str (& format ! ("// Token: {}\n" , token)) ; for path in paths . iter () . take (1) { if ! included_files . contains (path) { let include_path = path . strip_prefix ("../") . unwrap_or (path) ; graph_content . push_str (& format ! ("include!(\"{}\");\n" , include_path . display ())) ; included_files . insert (path . clone ()) ; } } graph_content . push ('\n') ; } let output_file = format ! ("../bootstrap3-incremental/{}_dependencies.rs" , bin_name) ; fs :: write (& output_file , graph_content) ? ; println ! ("✅ Dependency graph saved to: {}" , output_file) ; println ! ("📈 Total unique files needed: {}" , included_files . len ()) ; Ok (()) }
};
}
