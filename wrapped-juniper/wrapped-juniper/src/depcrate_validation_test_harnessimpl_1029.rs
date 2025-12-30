// Generated macro for impl_1029 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1029 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1029"}
// Dependencies: {}
impl < S > GraphQLType < S > for CatOrDog where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("CatOrDog")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let types = & [registry . get_type :: < Cat > (i) , registry . get_type :: < Dog > (i)] ; registry . build_union_type :: < Self > (i , types) . into_meta () } }
};
}
