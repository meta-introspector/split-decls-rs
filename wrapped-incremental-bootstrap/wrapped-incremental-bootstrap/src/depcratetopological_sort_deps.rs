// Generated macro for topological_sort_deps (function)
macro_rules! Depcratetopological_sort_deps {
() => {
// Module: crate
// Provides: {"topological_sort_deps"}
// Dependencies: {}
fn topological_sort_deps (tree : & HashMap < String , Vec < String > >) -> Result < Vec < String > , Box < dyn std :: error :: Error > > { let mut in_degree = HashMap :: new () ; let mut graph = HashMap :: new () ; for (node , deps) in tree { graph . insert (node . clone () , deps . clone ()) ; in_degree . entry (node . clone ()) . or_insert (0) ; for dep in deps { * in_degree . entry (dep . clone ()) . or_insert (0) += 1 ; } } let mut queue : Vec < _ > = in_degree . iter () . filter (| (_ , & degree) | degree == 0) . map (| (node , _) | node . clone ()) . collect () ; let mut result = Vec :: new () ; while let Some (node) = queue . pop () { result . push (node . clone ()) ; if let Some (deps) = graph . get (& node) { for dep in deps { if let Some (degree) = in_degree . get_mut (dep) { * degree -= 1 ; if * degree == 0 { queue . push (dep . clone ()) ; } } } } } Ok (result) }
};
}
