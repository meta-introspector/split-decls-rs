// Generated macro for impl_97 (impl)
macro_rules! Depcrate_item_scopeimpl_97 {
() => {
// Module: crate::item_scope
// Provides: {"impl_97"}
// Dependencies: {}
impl From < ImportOrExternCrate > for ImportOrDef { fn from (value : ImportOrExternCrate) -> Self { match value { ImportOrExternCrate :: Import (it) => ImportOrDef :: Import (it) , ImportOrExternCrate :: Glob (it) => ImportOrDef :: Glob (it) , ImportOrExternCrate :: ExternCrate (it) => ImportOrDef :: ExternCrate (it) , } } }
};
}
