// Generated macro for impl_524 (impl)
macro_rules! Depcrate_types_containersimpl_524 {
() => {
// Module: crate::types::containers
// Provides: {"impl_524"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for [T] where S : ScalarValue , T : GraphQLType < S > , { fn name (_ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { registry . build_list_type :: < T > (info , None) . into_meta () } }
};
}
