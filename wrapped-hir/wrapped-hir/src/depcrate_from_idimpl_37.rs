// Generated macro for impl_37 (impl)
macro_rules! Depcrate_from_idimpl_37 {
() => {
// Module: crate::from_id
// Provides: {"impl_37"}
// Dependencies: {}
impl From < DefWithBodyId > for DefWithBody { fn from (def : DefWithBodyId) -> Self { match def { DefWithBodyId :: FunctionId (it) => DefWithBody :: Function (it . into ()) , DefWithBodyId :: StaticId (it) => DefWithBody :: Static (it . into ()) , DefWithBodyId :: ConstId (it) => DefWithBody :: Const (it . into ()) , DefWithBodyId :: VariantId (it) => DefWithBody :: Variant (it . into ()) , } } }
};
}
