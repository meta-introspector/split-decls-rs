// Generated macro for impl_7309 (impl)
macro_rules! Depcrate_missing_asserts_for_indexingimpl_7309 {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"impl_7309"}
// Dependencies: {}
impl < 'hir > IndexEntry < 'hir > { pub fn slice (& self) -> & 'hir Expr < 'hir > { match self { IndexEntry :: StrayAssert { slice , .. } | IndexEntry :: AssertWithIndex { slice , .. } | IndexEntry :: IndexWithoutAssert { slice , .. } => slice , } } pub fn index_spans (& self) -> Option < & [Span] > { match self { IndexEntry :: StrayAssert { .. } => None , IndexEntry :: AssertWithIndex { indexes , .. } | IndexEntry :: IndexWithoutAssert { indexes , .. } => { Some (indexes) } , } } }
};
}
