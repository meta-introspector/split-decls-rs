macro_rules! deps {
    () => {
        Graph!();
        Commit!();
        Either!();
    };
}

macro_rules! LazyCommit {
    () => {
        deps!();
        # [doc = " A commit that provides access to graph-related information, on demand."] # [doc = ""] # [doc = " The owned version of this type is called [`Commit`] and can be obtained by calling [`LazyCommit::to_owned()`]."] pub struct LazyCommit < 'graph , 'cache > { backing : Either < & 'graph [u8] , (& 'cache gix_commitgraph :: Graph , gix_commitgraph :: Position) > , }
    };
}

LazyCommit!();