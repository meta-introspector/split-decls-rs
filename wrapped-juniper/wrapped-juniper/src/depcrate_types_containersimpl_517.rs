// Generated macro for impl_517 (impl)
macro_rules! Depcrate_types_containersimpl_517 {
() => {
// Module: crate::types::containers
// Provides: {"impl_517"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for Vec < T > where T : GraphQLType < S > , S : ScalarValue , { fn name (_ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { registry . build_list_type :: < T > (info , None) . into_meta () } }
};
}
