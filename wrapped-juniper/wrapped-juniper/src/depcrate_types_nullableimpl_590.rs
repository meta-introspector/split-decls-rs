// Generated macro for impl_590 (impl)
macro_rules! Depcrate_types_nullableimpl_590 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_590"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for Nullable < T > where T : GraphQLType < S > , S : ScalarValue , { fn name (_ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { registry . build_nullable_type :: < T > (info) . into_meta () } }
};
}
