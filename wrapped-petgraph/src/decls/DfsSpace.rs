macro_rules! deps {
    () => {
        Dfs!();
    };
}

macro_rules! DfsSpace {
    () => {
        deps!();
        # [doc = " Workspace for a graph traversal."] # [derive (Clone , Debug)] pub struct DfsSpace < N , VM > { dfs : Dfs < N , VM > , }
    };
}

DfsSpace!()