mkuse!{use rustc_index :: { Idx , IndexVec } ;}
mkuse!{use crate :: graph :: { DirectedGraph , NumEdges , Predecessors , Successors } ;}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkitem!{mkstruct!{# [doc = " A directed graph, efficient for cases where node indices are pre-existing."] # [doc = ""] # [doc = " If `BR` is true, the graph will store back-references, allowing you to get predecessors."] pub struct VecGraph < N : Idx , const BR : bool = false > { # [doc = " Indices into `edge_targets` that signify a start of list of edges."] node_starts : IndexVec < N , usize > , # [doc = " Targets (or sources for back refs) of edges"] edge_targets : Vec < N > , }}}
mkitem!{mkimpl!{impl < N : Idx + Ord , const BR : bool > VecGraph < N , BR > { pub fn new (num_nodes : usize , mut edge_pairs : Vec < (N , N) >) -> Self { let num_edges = edge_pairs . len () ; let nodes_cap = match BR { false => num_nodes + 1 , true => (num_nodes * 2) + 1 , } ; let edges_cap = match BR { false => num_edges , true => num_edges * 2 , } ; let mut node_starts = IndexVec :: with_capacity (nodes_cap) ; let mut edge_targets = Vec :: with_capacity (edges_cap) ; edge_pairs . sort () ; create_index (num_nodes , & mut edge_pairs . iter () . map (| & (src , _) | src) , & mut edge_pairs . iter () . map (| & (_ , tgt) | tgt) , & mut edge_targets , & mut node_starts ,) ; if BR { node_starts . pop () ; edge_pairs . sort_by_key (| & (src , tgt) | (tgt , src)) ; create_index (num_nodes * 2 , & mut edge_pairs . iter () . map (| & (_ , tgt) | N :: new (tgt . index () + num_nodes)) , & mut edge_pairs . iter () . map (| & (src , _) | src) , & mut edge_targets , & mut node_starts ,) ; } Self { node_starts , edge_targets } } # [doc = " Gets the successors for `source` as a slice."] pub fn successors (& self , source : N) -> & [N] { assert ! (source . index () < self . num_nodes ()) ; let start_index = self . node_starts [source] ; let end_index = self . node_starts [source . plus (1)] ; & self . edge_targets [start_index .. end_index] } }}}
mkitem!{mkimpl!{impl < N : Idx + Ord > VecGraph < N , true > { # [doc = " Gets the predecessors for `target` as a slice."] pub fn predecessors (& self , target : N) -> & [N] { assert ! (target . index () < self . num_nodes ()) ; let target = N :: new (target . index () + self . num_nodes ()) ; let start_index = self . node_starts [target] ; let end_index = self . node_starts [target . plus (1)] ; & self . edge_targets [start_index .. end_index] } }}}

macro_rules! create_index_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_index in module {}", module_path!());
    };
}

mkfn!{
    create_index_introspect!();
    # [doc = " Creates/initializes the index for the [`VecGraph`]. A helper for [`VecGraph::new`]."] # [doc = ""] # [doc = " - `num_nodes` is the target number of nodes in the graph"] # [doc = " - `sorted_edge_sources` are the edge sources, sorted"] # [doc = " - `associated_edge_targets` are the edge *targets* in the same order as sources"] # [doc = " - `edge_targets` is the vec of targets to be extended"] # [doc = " - `node_starts` is the index to be filled"] fn create_index < N : Idx + Ord > (num_nodes : usize , sorted_edge_sources : & mut dyn Iterator < Item = N > , associated_edge_targets : & mut dyn Iterator < Item = N > , edge_targets : & mut Vec < N > , node_starts : & mut IndexVec < N , usize > ,) { let offset = edge_targets . len () ; edge_targets . extend (associated_edge_targets) ; for (index , source) in sorted_edge_sources . enumerate () { while node_starts . len () <= source . index () { node_starts . push (index + offset) ; } } while node_starts . len () <= num_nodes { node_starts . push (edge_targets . len ()) ; } assert_eq ! (node_starts . len () , num_nodes + 1) ; }
}
mkitem!{mkimpl!{impl < N : Idx , const BR : bool > DirectedGraph for VecGraph < N , BR > { type Node = N ; fn num_nodes (& self) -> usize { match BR { false => self . node_starts . len () - 1 , true => (self . node_starts . len () - 1) / 2 , } } }}}
mkitem!{mkimpl!{impl < N : Idx , const BR : bool > NumEdges for VecGraph < N , BR > { fn num_edges (& self) -> usize { match BR { false => self . edge_targets . len () , true => self . edge_targets . len () / 2 , } } }}}
mkitem!{mkimpl!{impl < N : Idx + Ord , const BR : bool > Successors for VecGraph < N , BR > { fn successors (& self , node : N) -> impl Iterator < Item = Self :: Node > { self . successors (node) . iter () . cloned () } }}}
mkitem!{mkimpl!{impl < N : Idx + Ord > Predecessors for VecGraph < N , true > { fn predecessors (& self , node : Self :: Node) -> impl Iterator < Item = Self :: Node > { self . predecessors (node) . iter () . cloned () } }}}