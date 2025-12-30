// Generated macro for impl_36 (impl)
macro_rules! Depcrate_from_idimpl_36 {
() => {
// Module: crate::from_id
// Provides: {"impl_36"}
// Dependencies: {}
impl From < DefWithBody > for DefWithBodyId { fn from (def : DefWithBody) -> Self { match def { DefWithBody :: Function (it) => DefWithBodyId :: FunctionId (it . id) , DefWithBody :: Static (it) => DefWithBodyId :: StaticId (it . id) , DefWithBody :: Const (it) => DefWithBodyId :: ConstId (it . id) , DefWithBody :: Variant (it) => DefWithBodyId :: VariantId (it . into ()) , } } }
};
}
