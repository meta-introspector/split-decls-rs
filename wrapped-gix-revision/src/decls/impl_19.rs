macro_rules! deps {
    () => {
        Flags!();
        GenThenTime!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl From < & graph :: Commit < Flags > > for GenThenTime { fn from (commit : & graph :: Commit < Flags >) -> Self { GenThenTime { generation : commit . generation . unwrap_or (gix_commitgraph :: GENERATION_NUMBER_INFINITY) , time : commit . commit_time , } } }
    };
}

impl_19!();