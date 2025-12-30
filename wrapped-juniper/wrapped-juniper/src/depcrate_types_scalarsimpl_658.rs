// Generated macro for impl_658 (impl)
macro_rules! Depcrate_types_scalarsimpl_658 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_658"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for EmptySubscription < T > where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("_EmptySubscription")) } fn meta (_ : & () , registry : & mut Registry < S >) -> MetaType < S > { registry . build_object_type :: < Self > (& () , & []) . into_meta () } }
};
}
