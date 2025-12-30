// Generated macro for SingleThreaded (struct)
macro_rules! Depcrate_dbSingleThreaded {
() => {
// Module: crate::db
// Provides: {"SingleThreaded"}
// Dependencies: {}
# [doc = " Actual marker type for the marker trait `ThreadMode`, which holds"] # [doc = " a collection of column families without synchronization primitive, providing"] # [doc = " no overhead for the single-threaded column family alternations. The other"] # [doc = " mode is [`MultiThreaded`]."] # [doc = ""] # [doc = " See [`DB`] for more details, including performance implications for each mode"] pub struct SingleThreaded { pub (crate) cfs : BTreeMap < String , ColumnFamily > , }
};
}
