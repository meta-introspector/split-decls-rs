// Generated macro for impl_31 (impl)
macro_rules! Depcrate_merge_base_functionimpl_31 {
() => {
// Module: crate::merge_base::function
// Provides: {"impl_31"}
// Dependencies: {}
impl From < & graph :: Commit < Flags > > for GenThenTime { fn from (commit : & graph :: Commit < Flags >) -> Self { GenThenTime { generation : commit . generation . unwrap_or (gix_commitgraph :: GENERATION_NUMBER_INFINITY) , time : commit . commit_time , } } }
};
}
