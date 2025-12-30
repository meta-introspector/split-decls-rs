// Generated macro for Indexer (struct)
macro_rules! Depcrate_indexerIndexer {
() => {
// Module: crate::indexer
// Provides: {"Indexer"}
// Dependencies: {}
# [doc = " A stream to write and index a packfile"] # [doc = ""] # [doc = " This is equivalent to [`crate::OdbPackwriter`], but allows to store the pack"] # [doc = " and index at an arbitrary path. It also does not require access to an object"] # [doc = " database if, and only if, the pack file is self-contained (i.e. not \"thin\")."] pub struct Indexer < 'odb > { raw : * mut raw :: git_indexer , progress : raw :: git_indexer_progress , progress_payload_ptr : * mut OdbPackwriterCb < 'odb > , }
};
}
