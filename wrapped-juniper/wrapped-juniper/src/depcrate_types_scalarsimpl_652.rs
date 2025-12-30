// Generated macro for impl_652 (impl)
macro_rules! Depcrate_types_scalarsimpl_652 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_652"}
// Dependencies: {}
impl < S , T > GraphQLValue < S > for EmptyMutation < T > where S : ScalarValue , { type Context = T ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType < S > > :: name (info) } }
};
}
