macro_rules! deps {
    () => {
        GenAndCommitTime!();
        Queue!();
        Parents!();
    };
}

macro_rules! Topo {
    () => {
        deps!();
        # [doc = " A commit walker that walks in topographical order, like `git rev-list"] # [doc = " --topo-order` or `--date-order` depending on the chosen [`topo::Sorting`]."] # [doc = ""] # [doc = " Instantiate with [`topo::Builder`]."] pub struct Topo < Find , Predicate > { commit_graph : Option < gix_commitgraph :: Graph > , find : Find , predicate : Predicate , indegrees : IdMap < i32 > , states : IdMap < topo :: WalkFlags > , explore_queue : PriorityQueue < topo :: iter :: GenAndCommitTime , ObjectId > , indegree_queue : PriorityQueue < topo :: iter :: GenAndCommitTime , ObjectId > , topo_queue : topo :: iter :: Queue , parents : Parents , min_gen : u32 , buf : Vec < u8 > , }
    };
}

Topo!()