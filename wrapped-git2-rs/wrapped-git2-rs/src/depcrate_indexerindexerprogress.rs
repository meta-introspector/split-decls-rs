// Generated macro for IndexerProgress (type)
macro_rules! Depcrate_indexerIndexerProgress {
() => {
// Module: crate::indexer
// Provides: {"IndexerProgress"}
// Dependencies: {}
# [doc = " Callback to be invoked while indexing is in progress."] # [doc = ""] # [doc = " This callback will be periodically called with updates to the progress of"] # [doc = " the indexing so far. The return value indicates whether the indexing or"] # [doc = " transfer should continue. A return value of `false` will cancel the"] # [doc = " indexing or transfer."] # [doc = ""] # [doc = " * `progress` - the progress being made so far."] pub type IndexerProgress < 'a > = dyn FnMut (Progress < '_ >) -> bool + 'a ;
};
}
