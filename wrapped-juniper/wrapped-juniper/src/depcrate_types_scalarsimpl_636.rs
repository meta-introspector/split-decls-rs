// Generated macro for impl_636 (impl)
macro_rules! Depcrate_types_scalarsimpl_636 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_636"}
// Dependencies: {}
impl < S > GraphQLType < S > for str where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("String")) } fn meta (_ : & () , registry : & mut Registry < S >) -> MetaType < S > { registry . build_scalar_type :: < String > (& ()) . into_meta () } }
};
}
