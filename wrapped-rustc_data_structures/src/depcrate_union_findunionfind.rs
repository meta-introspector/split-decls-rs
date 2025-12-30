// Generated macro for UnionFind (struct)
macro_rules! Depcrate_union_findUnionFind {
() => {
// Module: crate::union_find
// Provides: {"UnionFind"}
// Dependencies: {}
# [doc = " Simple implementation of a union-find data structure, i.e. a disjoint-set"] # [doc = " forest."] # [derive (Debug)] pub struct UnionFind < Key : Idx > { table : IndexVec < Key , UnionFindEntry < Key > > , }
};
}
