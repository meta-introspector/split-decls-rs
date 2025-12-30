// Generated macro for impl_49 (impl)
macro_rules! Depcrate_from_idimpl_49 {
() => {
// Module: crate::from_id
// Provides: {"impl_49"}
// Dependencies: {}
impl From < hir_def :: item_scope :: ItemInNs > for ItemInNs { fn from (it : hir_def :: item_scope :: ItemInNs) -> Self { match it { hir_def :: item_scope :: ItemInNs :: Types (it) => ItemInNs :: Types (it . into ()) , hir_def :: item_scope :: ItemInNs :: Values (it) => ItemInNs :: Values (it . into ()) , hir_def :: item_scope :: ItemInNs :: Macros (it) => ItemInNs :: Macros (it . into ()) , } } }
};
}
