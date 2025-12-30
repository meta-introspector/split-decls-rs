// Generated macro for impl_1610 (impl)
macro_rules! Depcrate_utils_step_graphimpl_1610 {
() => {
// Module: crate::utils::step_graph
// Provides: {"impl_1610"}
// Dependencies: {}
impl DotGraph { fn add_node (& mut self , key : String , node : Node) -> NodeHandle { let handle = NodeHandle (self . nodes . len ()) ; self . nodes . push (node) ; self . key_to_index . insert (key , handle) ; handle } fn add_edge (& mut self , src : NodeHandle , dst : NodeHandle) { self . edges . insert (Edge { src , dst , cached : false }) ; } fn add_cached_edge (& mut self , src : NodeHandle , dst : NodeHandle) { let uncached = Edge { src , dst , cached : false } ; if ! self . edges . contains (& uncached) { self . edges . insert (Edge { src , dst , cached : true }) ; } } fn get_handle_by_key (& self , key : & str) -> Option < NodeHandle > { self . key_to_index . get (key) . copied () } fn render (& self , path : & Path) -> std :: io :: Result < () > { use std :: io :: Write ; let mut file = BufWriter :: new (std :: fs :: File :: create (path) ?) ; writeln ! (file , "digraph bootstrap_steps {{") ? ; for (index , node) in self . nodes . iter () . enumerate () { writeln ! (file , r#"{index} [label="{}", tooltip="{}"]"# , escape (& node . label) , escape (& node . tooltip)) ? ; } let mut edges : Vec < & Edge > = self . edges . iter () . collect () ; edges . sort () ; for edge in edges { let style = if edge . cached { "dashed" } else { "solid" } ; writeln ! (file , r#"{} -> {} [style="{style}"]"# , edge . src . 0 , edge . dst . 0) ? ; } writeln ! (file , "}}") } }
};
}
