// Generated macro for impl_1039 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1039 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1039"}
// Dependencies: {}
impl < S > GraphQLType < S > for HumanOrAlien where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("HumanOrAlien")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let types = & [registry . get_type :: < Human > (i) , registry . get_type :: < Alien > (i)] ; registry . build_union_type :: < Self > (i , types) . into_meta () } }
};
}
