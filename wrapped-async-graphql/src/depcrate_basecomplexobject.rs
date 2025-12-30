// Generated macro for ComplexObject (trait)
macro_rules! Depcrate_baseComplexObject {
() => {
// Module: crate::base
// Provides: {"ComplexObject"}
// Dependencies: {}
# [doc (hidden)] # [cfg_attr (feature = "boxed-trait" , async_trait :: async_trait)] pub trait ComplexObject { fn fields (registry : & mut registry :: Registry) -> Vec < (String , registry :: MetaField) > ; # [cfg (feature = "boxed-trait")] async fn resolve_field (& self , ctx : & Context < '_ >) -> ServerResult < Option < Value > > ; # [cfg (not (feature = "boxed-trait"))] fn resolve_field (& self , ctx : & Context < '_ > ,) -> impl Future < Output = ServerResult < Option < Value > > > + Send ; }
};
}
