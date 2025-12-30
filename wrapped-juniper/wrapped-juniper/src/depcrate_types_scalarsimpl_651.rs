// Generated macro for impl_651 (impl)
macro_rules! Depcrate_types_scalarsimpl_651 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_651"}
// Dependencies: {}
impl < S , T > GraphQLType < S > for EmptyMutation < T > where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("_EmptyMutation")) } fn meta (_ : & () , registry : & mut Registry < S >) -> MetaType < S > { registry . build_object_type :: < Self > (& () , & []) . into_meta () } }
};
}
