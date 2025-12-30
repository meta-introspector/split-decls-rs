// Generated macro for impl_612 (impl)
macro_rules! Depcrate_types_pointersimpl_612 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_612"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for Arc < T > where S : ScalarValue , T : GraphQLType < S > + ? Sized , { fn name (info : & Self :: TypeInfo) -> Option < ArcStr > { T :: name (info) } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { T :: meta (info , registry) } }
};
}
