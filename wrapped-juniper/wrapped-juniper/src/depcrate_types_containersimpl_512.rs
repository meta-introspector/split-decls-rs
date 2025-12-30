// Generated macro for impl_512 (impl)
macro_rules! Depcrate_types_containersimpl_512 {
() => {
// Module: crate::types::containers
// Provides: {"impl_512"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for Option < T > where T : GraphQLType < S > , S : ScalarValue , { fn name (_ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { registry . build_nullable_type :: < T > (info) . into_meta () } }
};
}
