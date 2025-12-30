// Generated macro for ScopedHashMap (struct)
macro_rules! Depcrate_scoped_hash_mapScopedHashMap {
() => {
// Module: crate::scoped_hash_map
// Provides: {"ScopedHashMap"}
// Dependencies: {}
# [doc = " A wrapper around a `FxHashMap` which adds the concept of scopes. Items inserted"] # [doc = " within a scope are removed when the scope is exited."] # [doc = ""] # [doc = " Shadowing, where one scope has entries with the same keys as a containing scope,"] # [doc = " is not supported in this implementation."] pub struct ScopedHashMap < K , V > { map : CtxHashMap < K , Val < V > > , generation_by_depth : SmallVec < [u32 ; 8] > , generation : u32 , }
};
}
