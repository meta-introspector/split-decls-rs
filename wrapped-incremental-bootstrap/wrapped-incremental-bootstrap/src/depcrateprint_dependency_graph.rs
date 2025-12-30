// Generated macro for print_dependency_graph (function)
macro_rules! Depcrateprint_dependency_graph {
() => {
// Module: crate
// Provides: {"print_dependency_graph"}
// Dependencies: {}
fn print_dependency_graph (bin_name : & str , max_depth : usize) -> Result < () , Box < dyn std :: error :: Error > > { println ! ("📊 Dependency Graph for: {}" , bin_name) ; println ! ("═══════════════════════════════════════") ; let mut cache = DepCache :: new () ; let mut processed = HashSet :: new () ; let mut to_process = Vec :: new () ; let bin_path = find_binary_in_output2 (bin_name) ? ; let root_id = cache . get_or_compute (& bin_path) ? ; to_process . push ((root_id . clone () , 0)) ; let mut dependency_tree = HashMap :: new () ; while let Some ((current_id , depth)) = to_process . pop () { if processed . contains (& current_id) || depth >= max_depth { continue ; } processed . insert (current_id . clone ()) ; let node = cache . nodes . get (& current_id) . unwrap () ; let mut resolved_deps = HashMap :: new () ; scan_output2_for_tokens (& node . tokens , & mut resolved_deps) ? ; let mut dep_ids = Vec :: new () ; for (_token , paths) in resolved_deps { if let Some (first_path) = paths . first () { let dep_id = cache . get_or_compute (first_path) ? ; dep_ids . push (dep_id . clone ()) ; to_process . push ((dep_id , depth + 1)) ; } } dependency_tree . insert (current_id . clone () , dep_ids) ; } print_tree (& root_id , & dependency_tree , & cache , 0 , & mut HashSet :: new ()) ? ; println ! ("\n📈 Graph Summary:") ; println ! ("   Total nodes: {}" , processed . len ()) ; println ! ("   Max depth: {}" , max_depth) ; println ! ("   Cache entries: {}" , cache . nodes . len ()) ; Ok (()) }
};
}
