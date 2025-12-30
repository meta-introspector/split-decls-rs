// Generated macro for impl_1037 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1037 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1037"}
// Dependencies: {}
impl < S > GraphQLType < S > for DogOrHuman where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("DogOrHuman")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let types = & [registry . get_type :: < Dog > (i) , registry . get_type :: < Human > (i)] ; registry . build_union_type :: < Self > (i , types) . into_meta () } }
};
}
