macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! IdMap {
    () => {
        deps!();
        # [doc = " A map associating an object id with its commit-metadata."] pub type IdMap = gix_revwalk :: graph :: IdMap < gix_revwalk :: graph :: Commit < Metadata > > ;
    };
}

IdMap!();