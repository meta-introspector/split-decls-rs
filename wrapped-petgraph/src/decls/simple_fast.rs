macro_rules! deps {
    () => {
        Time!();
        Dominators!();
    };
}

macro_rules! simple_fast {
    () => {
        deps!();
        # [doc = " This is an implementation of the engineered [\"Simple, Fast Dominance"] # [doc = " Algorithm\"][0] discovered by Cooper et al."] # [doc = ""] # [doc = " This algorithm is **O(|V|²)** where V is the set of nodes, and therefore has slower theoretical running time"] # [doc = " than the Lengauer-Tarjan algorithm (which is **O(|E| log |V|)** where E is the set of edges). However,"] # [doc = " Cooper et al found it to be faster in practice on control flow graphs of up"] # [doc = " to ~30,000 nodes."] # [doc = ""] # [doc = " # Arguments"] # [doc = " * `graph`: a control-flow graph."] # [doc = " * `root`: the *root* node of the `graph`."] # [doc = ""] # [doc = " # Returns"] # [doc = " * `Dominators`: the dominance relation for given `graph` and `root`"] # [doc = "   represented by [`struct@Dominators`]."] # [doc = ""] # [doc = " # Complexity"] # [doc = " * Time complexity: **O(|V|²)**."] # [doc = " * Auxiliary space: **O(|V| + |E|)**."] # [doc = ""] # [doc = " where **|V|** is the number of nodes and **|E|** is the number of edges."] # [doc = ""] # [doc = " [0]: http://www.hipersoft.rice.edu/grads/publications/dom14.pdf"] pub fn simple_fast < G > (graph : G , root : G :: NodeId) -> Dominators < G :: NodeId > where G : IntoNeighbors + Visitable , < G as GraphBase > :: NodeId : Eq + Hash , { let (post_order , predecessor_sets) = simple_fast_post_order (graph , root) ; let length = post_order . len () ; debug_assert ! (length > 0) ; debug_assert ! (post_order . last () == Some (& root)) ; let node_to_post_order_idx : HashMap < _ , _ > = post_order . iter () . enumerate () . map (| (idx , & node) | (node , idx)) . collect () ; let idx_to_predecessor_vec = predecessor_sets_to_idx_vecs (& post_order , & node_to_post_order_idx , predecessor_sets) ; let mut dominators = vec ! [UNDEFINED ; length] ; dominators [length - 1] = length - 1 ; let mut changed = true ; while changed { changed = false ; for idx in (0 .. length - 1) . rev () { debug_assert ! (post_order [idx] != root) ; let new_idom_idx = { let mut predecessors = idx_to_predecessor_vec [idx] . iter () . filter (| & & p | dominators [p] != UNDEFINED) ; let new_idom_idx = predecessors . next () . expect ("Because the root is initialized to dominate itself, and is the \
                     first node in every path, there must exist a predecessor to this \
                     node that also has a dominator" ,) ; predecessors . fold (* new_idom_idx , | new_idom_idx , & predecessor_idx | { intersect (& dominators , new_idom_idx , predecessor_idx) }) } ; debug_assert ! (new_idom_idx < length) ; if new_idom_idx != dominators [idx] { dominators [idx] = new_idom_idx ; changed = true ; } } } debug_assert ! (! dominators . contains (& UNDEFINED)) ; Dominators { root , dominators : dominators . into_iter () . enumerate () . map (| (idx , dom_idx) | (post_order [idx] , post_order [dom_idx])) . collect () , } }
    };
}

simple_fast!()