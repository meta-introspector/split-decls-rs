// Generated macro for impl_31 (impl)
macro_rules! Depcrate_from_idimpl_31 {
() => {
// Module: crate::from_id
// Provides: {"impl_31"}
// Dependencies: {}
impl From < GenericParam > for GenericParamId { fn from (id : GenericParam) -> Self { match id { GenericParam :: LifetimeParam (it) => GenericParamId :: LifetimeParamId (it . id) , GenericParam :: ConstParam (it) => GenericParamId :: ConstParamId (it . id) , GenericParam :: TypeParam (it) => GenericParamId :: TypeParamId (it . id) , } } }
};
}
