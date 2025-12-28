macro_rules! deps {
    () => {
        DfsSpace!();
    };
}

macro_rules! DfsSpaceType {
    () => {
        deps!();
        type DfsSpaceType < G > = DfsSpace < < G as GraphBase > :: NodeId , < G as Visitable > :: Map > ;
    };
}

DfsSpaceType!();