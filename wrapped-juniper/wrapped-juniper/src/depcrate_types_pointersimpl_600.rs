// Generated macro for impl_600 (impl)
macro_rules! Depcrate_types_pointersimpl_600 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_600"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for Box < T > where T : GraphQLType < S > + ? Sized , S : ScalarValue , { fn name (info : & Self :: TypeInfo) -> Option < ArcStr > { T :: name (info) } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { T :: meta (info , registry) } }
};
}
