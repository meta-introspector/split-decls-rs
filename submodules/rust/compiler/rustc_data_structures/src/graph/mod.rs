mkuse!{use rustc_index :: Idx ;}
mkmod!{dominators, { 
                getname!(dominators);
                getsrc!(dominators);
                getpath!(dominators);
                get_deps!(dominators);
                get_crates!(dominators);
                mkinclude!(dominators);
                 
            }}
mkmod!{iterate, { 
                getname!(iterate);
                getsrc!(iterate);
                getpath!(iterate);
                get_deps!(iterate);
                get_crates!(iterate);
                mkinclude!(iterate);
                 
            }}
mkmod!{linked_graph, { 
                getname!(linked_graph);
                getsrc!(linked_graph);
                getpath!(linked_graph);
                get_deps!(linked_graph);
                get_crates!(linked_graph);
                mkinclude!(linked_graph);
                 
            }}
mkmod!{reference, { 
                getname!(reference);
                getsrc!(reference);
                getpath!(reference);
                get_deps!(reference);
                get_crates!(reference);
                mkinclude!(reference);
                 
            }}
mkmod!{reversed, { 
                getname!(reversed);
                getsrc!(reversed);
                getpath!(reversed);
                get_deps!(reversed);
                get_crates!(reversed);
                mkinclude!(reversed);
                 
            }}
mkmod!{scc, { 
                getname!(scc);
                getsrc!(scc);
                getpath!(scc);
                get_deps!(scc);
                get_crates!(scc);
                mkinclude!(scc);
                 
            }}
mkmod!{vec_graph, { 
                getname!(vec_graph);
                getsrc!(vec_graph);
                getpath!(vec_graph);
                get_deps!(vec_graph);
                get_crates!(vec_graph);
                mkinclude!(vec_graph);
                 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mktrait!{pub trait DirectedGraph { type Node : Idx ; # [doc = " Returns the total number of nodes in this graph."] # [doc = ""] # [doc = " Several graph algorithm implementations assume that every node ID is"] # [doc = " strictly less than the number of nodes, i.e. nodes are densely numbered."] # [doc = " That assumption allows them to use `num_nodes` to allocate per-node"] # [doc = " data structures, indexed by node."] fn num_nodes (& self) -> usize ; # [doc = " Iterates over all nodes of a graph in ascending numeric order."] # [doc = ""] # [doc = " Assumes that nodes are densely numbered, i.e. every index in"] # [doc = " `0..num_nodes` is a valid node."] fn iter_nodes (& self ,) -> impl Iterator < Item = Self :: Node > + DoubleEndedIterator + ExactSizeIterator { (0 .. self . num_nodes ()) . map (< Self :: Node as Idx > :: new) } }}}
mkitem!{mktrait!{pub trait NumEdges : DirectedGraph { fn num_edges (& self) -> usize ; }}}
mkitem!{mktrait!{pub trait StartNode : DirectedGraph { fn start_node (& self) -> Self :: Node ; }}}
mkitem!{mktrait!{pub trait Successors : DirectedGraph { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }}}
mkitem!{mktrait!{pub trait Predecessors : DirectedGraph { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > ; }}}
mkitem!{mktrait!{# [doc = " Alias for [`DirectedGraph`] + [`StartNode`] + [`Predecessors`] + [`Successors`]."] pub trait ControlFlowGraph : DirectedGraph + StartNode + Predecessors + Successors { }}}
mkitem!{mkimpl!{impl < T > ControlFlowGraph for T where T : DirectedGraph + StartNode + Predecessors + Successors { }}}

macro_rules! is_cyclic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_cyclic in module {}", module_path!());
    };
}

mkfn!{
    is_cyclic_introspect!();
    # [doc = " Returns `true` if the graph has a cycle that is reachable from the start node."] pub fn is_cyclic < G > (graph : & G) -> bool where G : ? Sized + DirectedGraph + StartNode + Successors , { iterate :: TriColorDepthFirstSearch :: new (graph) . run_from_start (& mut iterate :: CycleDetector) . is_some () }
}

macro_rules! depth_first_search_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function depth_first_search in module {}", module_path!());
    };
}

mkfn!{
    depth_first_search_introspect!();
    pub fn depth_first_search < G > (graph : G , from : G :: Node) -> iterate :: DepthFirstSearch < G > where G : Successors , { iterate :: DepthFirstSearch :: new (graph) . with_start_node (from) }
}

macro_rules! depth_first_search_as_undirected_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function depth_first_search_as_undirected in module {}", module_path!());
    };
}

mkfn!{
    depth_first_search_as_undirected_introspect!();
    pub fn depth_first_search_as_undirected < G > (graph : G , from : G :: Node ,) -> iterate :: DepthFirstSearch < impl Successors < Node = G :: Node > > where G : Successors + Predecessors , { struct AsUndirected < G > (G) ; impl < G : DirectedGraph > DirectedGraph for AsUndirected < G > { type Node = G :: Node ; fn num_nodes (& self) -> usize { self . 0 . num_nodes () } } impl < G : Successors + Predecessors > Successors for AsUndirected < G > { fn successors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . 0 . successors (node) . chain (self . 0 . predecessors (node)) } } iterate :: DepthFirstSearch :: new (AsUndirected (graph)) . with_start_node (from) }
}