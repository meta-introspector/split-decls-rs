macro_rules! deps {
    () => {
        Graph!();
        Either!();
    };
}

macro_rules! Parents {
    () => {
        deps!();
        # [doc = " An iterator over the parents of a commit."] pub struct Parents < 'graph , 'cache > { backing : Either < gix_object :: CommitRefIter < 'graph > , (& 'cache gix_commitgraph :: Graph , gix_commitgraph :: file :: commit :: Parents < 'cache > ,) , > , }
    };
}

Parents!();