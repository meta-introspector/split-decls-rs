// Generated macro for Algorithm (enum)
macro_rules! Depcrate_index_traverse_typesAlgorithm {
() => {
// Module: crate::index::traverse::types
// Provides: {"Algorithm"}
// Dependencies: {}
# [doc = " The way we verify the pack"] # [derive (Default , Debug , PartialEq , Eq , Hash , Ord , PartialOrd , Clone , Copy)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub enum Algorithm { # [doc = " Build an index to allow decoding each delta and base exactly once, saving a lot of computational"] # [doc = " resource at the expense of resident memory, as we will use an additional `DeltaTree` to accelerate"] # [doc = " delta chain resolution."] # [default] DeltaTreeLookup , # [doc = " We lookup each object similarly to what would happen during normal repository use."] # [doc = " Uses more compute resources as it will resolve delta chains from back to front, but start right away"] # [doc = " without indexing or investing any memory in indices."] # [doc = ""] # [doc = " This option may be well suited for big packs in memory-starved system that support memory mapping."] Lookup , }
};
}
