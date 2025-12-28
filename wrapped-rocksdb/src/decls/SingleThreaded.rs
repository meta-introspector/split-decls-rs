macro_rules! deps {
    () => {
        ThreadMode!();
        DB!();
        ColumnFamily!();
        MultiThreaded!();
    };
}

macro_rules! SingleThreaded {
    () => {
        deps!();
        # [doc = " Actual marker type for the marker trait `ThreadMode`, which holds"] # [doc = " a collection of column families without synchronization primitive, providing"] # [doc = " no overhead for the single-threaded column family alternations. The other"] # [doc = " mode is [`MultiThreaded`]."] # [doc = ""] # [doc = " See [`DB`] for more details, including performance implications for each mode"] pub struct SingleThreaded { pub (crate) cfs : BTreeMap < String , ColumnFamily > , }
    };
}

SingleThreaded!();