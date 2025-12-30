// Generated macro for impl_607 (impl)
macro_rules! Depcrate_types_pointersimpl_607 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_607"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for & T where T : GraphQLType < S > + ? Sized , S : ScalarValue , { fn name (info : & Self :: TypeInfo) -> Option < ArcStr > { T :: name (info) } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { T :: meta (info , registry) } }
};
}
