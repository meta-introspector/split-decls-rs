// Generated macro for impl_92 (impl)
macro_rules! Depcrate_item_scopeimpl_92 {
() => {
// Module: crate::item_scope
// Provides: {"impl_92"}
// Dependencies: {}
impl From < ImportOrGlob > for ImportOrExternCrate { fn from (value : ImportOrGlob) -> Self { match value { ImportOrGlob :: Glob (it) => ImportOrExternCrate :: Glob (it) , ImportOrGlob :: Import (it) => ImportOrExternCrate :: Import (it) , } } }
};
}
