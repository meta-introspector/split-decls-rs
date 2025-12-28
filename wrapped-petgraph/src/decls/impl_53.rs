macro_rules! deps {
    () => {
        Create!();
        VisitMap!();
        Reversed!();
        GraphRef!();
        Topo!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < N , VM > Topo < N , VM > where N : Copy + PartialEq , VM : VisitMap < N > , { # [doc = " Create a new `Topo`, using the graph's visitor map, and put all"] # [doc = " initial nodes in the to visit list."] pub fn new < G > (graph : G) -> Self where G : IntoNodeIdentifiers + IntoNeighborsDirected + Visitable < NodeId = N , Map = VM > , { let mut topo = Self :: empty (graph) ; topo . extend_with_initials (graph) ; topo } # [doc = " Create a new `Topo` with initial nodes."] # [doc = ""] # [doc = " Nodes with incoming edges are ignored."] pub fn with_initials < G , I > (graph : G , initials : I) -> Self where G : IntoNeighborsDirected + Visitable < NodeId = N , Map = VM > , I : IntoIterator < Item = N > , { Topo { tovisit : initials . into_iter () . filter (| & n | graph . neighbors_directed (n , Incoming) . next () . is_none ()) . collect () , ordered : graph . visit_map () , } } fn extend_with_initials < G > (& mut self , g : G) where G : IntoNodeIdentifiers + IntoNeighborsDirected < NodeId = N > , { self . tovisit . extend (g . node_identifiers () . filter (move | & a | g . neighbors_directed (a , Incoming) . next () . is_none ()) ,) ; } # [doc = " Create a new `Topo`, using the graph's visitor map with *no* starting"] # [doc = " index specified."] fn empty < G > (graph : G) -> Self where G : GraphRef + Visitable < NodeId = N , Map = VM > , { Topo { ordered : graph . visit_map () , tovisit : Vec :: new () , } } # [doc = " Clear visited state, and put all initial nodes in the to visit list."] pub fn reset < G > (& mut self , graph : G) where G : IntoNodeIdentifiers + IntoNeighborsDirected + Visitable < NodeId = N , Map = VM > , { graph . reset_map (& mut self . ordered) ; self . tovisit . clear () ; self . extend_with_initials (graph) ; } # [doc = " Return the next node in the current topological order traversal, or"] # [doc = " `None` if the traversal is at the end."] # [doc = ""] # [doc = " *Note:* The graph may not have a complete topological order, and the only"] # [doc = " way to know is to run the whole traversal and make sure it visits every node."] pub fn next < G > (& mut self , g : G) -> Option < N > where G : IntoNeighborsDirected + Visitable < NodeId = N , Map = VM > , { while let Some (nix) = self . tovisit . pop () { if self . ordered . is_visited (& nix) { continue ; } self . ordered . visit (nix) ; for neigh in g . neighbors (nix) { if Reversed (g) . neighbors (neigh) . all (| b | self . ordered . is_visited (& b)) { self . tovisit . push (neigh) ; } } return Some (nix) ; } None } }
    };
}

impl_53!();