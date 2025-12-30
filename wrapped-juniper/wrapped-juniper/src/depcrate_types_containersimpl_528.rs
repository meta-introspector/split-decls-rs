// Generated macro for impl_528 (impl)
macro_rules! Depcrate_types_containersimpl_528 {
() => {
// Module: crate::types::containers
// Provides: {"impl_528"}
// Dependencies: {}
impl < S , T , const N : usize > GraphQLType < S > for [T ; N] where S : ScalarValue , T : GraphQLType < S > , { fn name (_ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { registry . build_list_type :: < T > (info , Some (N)) . into_meta () } }
};
}
