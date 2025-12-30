// Generated macro for impl_30 (impl)
macro_rules! Depcrate_from_idimpl_30 {
() => {
// Module: crate::from_id
// Provides: {"impl_30"}
// Dependencies: {}
impl From < GenericParamId > for GenericParam { fn from (id : GenericParamId) -> Self { match id { GenericParamId :: TypeParamId (it) => GenericParam :: TypeParam (it . into ()) , GenericParamId :: ConstParamId (it) => GenericParam :: ConstParam (it . into ()) , GenericParamId :: LifetimeParamId (it) => GenericParam :: LifetimeParam (it . into ()) , } } }
};
}
