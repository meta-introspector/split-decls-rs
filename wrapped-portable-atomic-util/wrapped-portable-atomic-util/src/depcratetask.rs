// Generated macro for task (module)
macro_rules! Depcratetask {
() => {
// Module: crate
// Provides: {"task"}
// Dependencies: {}
# [cfg (not (portable_atomic_no_futures_api))] # [cfg (any (all (feature = "alloc" , not (portable_atomic_no_alloc)) , feature = "std"))] # [cfg_attr (docsrs , doc (cfg (any (feature = "alloc" , feature = "std"))))] pub mod task ;
};
}
