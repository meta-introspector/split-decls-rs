macro_rules! deps {
    () => {
        Sorting!();
    };
}

macro_rules! impl_858 {
    () => {
        deps!();
        impl Sorting { fn into_simple (self) -> Option < gix_traverse :: commit :: simple :: Sorting > { Some (match self { Sorting :: BreadthFirst => gix_traverse :: commit :: simple :: Sorting :: BreadthFirst , Sorting :: ByCommitTime (order) => gix_traverse :: commit :: simple :: Sorting :: ByCommitTime (order) , Sorting :: ByCommitTimeCutoff { seconds , order } => { gix_traverse :: commit :: simple :: Sorting :: ByCommitTimeCutoff { order , seconds } } }) } }
    };
}

impl_858!()