macro_rules! deps {
    () => {
        GraphRef!();
        Dfs!();
        DfsSpace!();
        VisitMap!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < N , VM > DfsSpace < N , VM > where N : Copy + PartialEq , VM : VisitMap < N > , { pub fn new < G > (g : G) -> Self where G : GraphRef + Visitable < NodeId = N , Map = VM > , { DfsSpace { dfs : Dfs :: empty (g) } } }
    };
}

impl_483!()