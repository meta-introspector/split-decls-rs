macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! DfsEvent {
    () => {
        deps!();
        # [doc = " A depth first search (DFS) visitor event."] # [derive (Copy , Clone , Debug)] pub enum DfsEvent < N > { Discover (N , Time) , # [doc = " An edge of the tree formed by the traversal."] TreeEdge (N , N) , # [doc = " An edge to an already visited node."] BackEdge (N , N) , # [doc = " A cross or forward edge."] # [doc = ""] # [doc = " For an edge *(u, v)*, if the discover time of *v* is greater than *u*,"] # [doc = " then it is a forward edge, else a cross edge."] CrossForwardEdge (N , N) , # [doc = " All edges from a node have been reported."] Finish (N , Time) , }
    };
}

DfsEvent!();