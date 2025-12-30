// Generated macro for BlockBasedTablePinningTier (enum)
macro_rules! Depcrate_db_optionsBlockBasedTablePinningTier {
() => {
// Module: crate::db_options
// Provides: {"BlockBasedTablePinningTier"}
// Dependencies: {}
# [doc = " Used by BlockBasedOptions for setting metadata cache pinning tiers."] # [doc = " Controls how metadata blocks (index, filter, etc.) are pinned in block cache."] # [repr (C)] pub enum BlockBasedTablePinningTier { # [doc = " Use fallback pinning tier (context-dependent)"] Fallback = ffi :: rocksdb_block_based_k_fallback_pinning_tier as isize , # [doc = " No pinning - blocks can be evicted at any time"] None = ffi :: rocksdb_block_based_k_none_pinning_tier as isize , # [doc = " Pin blocks for flushed files and similar scenarios"] FlushAndSimilar = ffi :: rocksdb_block_based_k_flush_and_similar_pinning_tier as isize , # [doc = " Pin all blocks (highest priority)"] All = ffi :: rocksdb_block_based_k_all_pinning_tier as isize , }
};
}
