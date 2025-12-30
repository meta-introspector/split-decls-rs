// Generated macro for impl_50 (impl)
macro_rules! Depcrate_from_idimpl_50 {
() => {
// Module: crate::from_id
// Provides: {"impl_50"}
// Dependencies: {}
impl From < ItemInNs > for hir_def :: item_scope :: ItemInNs { fn from (it : ItemInNs) -> Self { match it { ItemInNs :: Types (it) => Self :: Types (it . into ()) , ItemInNs :: Values (it) => Self :: Values (it . into ()) , ItemInNs :: Macros (it) => Self :: Macros (it . into ()) , } } }
};
}
