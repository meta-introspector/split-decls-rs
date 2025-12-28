macro_rules! DfsPostOrder {
    () => {
        # [doc = " Visit nodes in a depth-first-search (DFS) emitting nodes in postorder"] # [doc = " (each node after all its descendants have been emitted)."] # [doc = ""] # [doc = " `DfsPostOrder` is not recursive."] # [doc = ""] # [doc = " The traversal starts at a given node and only traverses nodes reachable"] # [doc = " from it."] # [derive (Clone , Debug)] pub struct DfsPostOrder < N , VM > { # [doc = " The stack of nodes to visit"] pub stack : Vec < N > , # [doc = " The map of discovered nodes"] pub discovered : VM , # [doc = " The map of finished nodes"] pub finished : VM , }
    };
}

DfsPostOrder!();