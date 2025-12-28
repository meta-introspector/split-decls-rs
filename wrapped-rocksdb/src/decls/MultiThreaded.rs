macro_rules! deps {
    () => {
        ThreadMode!();
        UnboundColumnFamily!();
        SingleThreaded!();
        DB!();
    };
}

macro_rules! MultiThreaded {
    () => {
        deps!();
        # [doc = " Actual marker type for the marker trait `ThreadMode`, which holds"] # [doc = " a collection of column families wrapped in a RwLock to be mutated"] # [doc = " concurrently. The other mode is [`SingleThreaded`]."] # [doc = ""] # [doc = " See [`DB`] for more details, including performance implications for each mode"] pub struct MultiThreaded { pub (crate) cfs : RwLock < BTreeMap < String , Arc < UnboundColumnFamily > > > , }
    };
}

MultiThreaded!()