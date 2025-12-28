macro_rules! deps {
    () => {
        DfsSpaceType!();
        GraphRef!();
        Create!();
        Dfs!();
    };
}

macro_rules! with_dfs {
    () => {
        deps!();
        # [doc = " Create a Dfs if it's needed"] fn with_dfs < G , F , R > (g : G , space : Option < & mut DfsSpaceType < G > > , f : F) -> R where G : GraphRef + Visitable , F : FnOnce (& mut Dfs < G :: NodeId , G :: Map >) -> R , { let mut local_visitor ; let dfs = if let Some (v) = space { & mut v . dfs } else { local_visitor = Dfs :: empty (g) ; & mut local_visitor } ; f (dfs) }
    };
}

with_dfs!()