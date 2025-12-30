// Generated macro for impl_659 (impl)
macro_rules! Depcrate_types_scalarsimpl_659 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_659"}
// Dependencies: {}
impl < S , T > GraphQLValue < S > for EmptySubscription < T > where S : ScalarValue , { type Context = T ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType < S > > :: name (info) } }
};
}
