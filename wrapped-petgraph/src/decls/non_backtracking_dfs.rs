macro_rules! non_backtracking_dfs {
    () => {
        fn non_backtracking_dfs < G , F > (graph : & G , source : G :: NodeId , visited : & mut G :: Map , mut visitor : F) where G : Visitable + IntoNeighbors , F : FnMut (G :: NodeId) , { if visited . visit (source) { for target in graph . neighbors (source) { if ! visited . is_visited (& target) { visitor (target) ; non_backtracking_dfs (graph , target , visited , visitor) ; break ; } } } }
    };
}

non_backtracking_dfs!()