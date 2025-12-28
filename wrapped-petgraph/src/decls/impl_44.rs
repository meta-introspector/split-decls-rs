macro_rules! deps {
    () => {
        VisitMap!();
        GraphRef!();
        Dfs!();
        Create!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl < N , VM > Dfs < N , VM > where N : Copy + PartialEq , VM : VisitMap < N > , { # [doc = " Create a new **Dfs**, using the graph's visitor map, and put **start**"] # [doc = " in the stack of nodes to visit."] pub fn new < G > (graph : G , start : N) -> Self where G : GraphRef + Visitable < NodeId = N , Map = VM > , { let mut dfs = Dfs :: empty (graph) ; dfs . move_to (start) ; dfs } # [doc = " Create a `Dfs` from a vector and a visit map"] pub fn from_parts (stack : Vec < N > , discovered : VM) -> Self { Dfs { stack , discovered } } # [doc = " Clear the visit state"] pub fn reset < G > (& mut self , graph : G) where G : GraphRef + Visitable < NodeId = N , Map = VM > , { graph . reset_map (& mut self . discovered) ; self . stack . clear () ; } # [doc = " Create a new **Dfs** using the graph's visitor map, and no stack."] pub fn empty < G > (graph : G) -> Self where G : GraphRef + Visitable < NodeId = N , Map = VM > , { Dfs { stack : Vec :: new () , discovered : graph . visit_map () , } } # [doc = " Keep the discovered map, but clear the visit stack and restart"] # [doc = " the dfs from a particular node."] pub fn move_to (& mut self , start : N) { self . stack . clear () ; self . stack . push (start) ; } # [doc = " Return the next node in the dfs, or **None** if the traversal is done."] pub fn next < G > (& mut self , graph : G) -> Option < N > where G : IntoNeighbors < NodeId = N > , { while let Some (node) = self . stack . pop () { if self . discovered . visit (node) { for succ in graph . neighbors (node) { if ! self . discovered . is_visited (& succ) { self . stack . push (succ) ; } } return Some (node) ; } } None } }
    };
}

impl_44!()