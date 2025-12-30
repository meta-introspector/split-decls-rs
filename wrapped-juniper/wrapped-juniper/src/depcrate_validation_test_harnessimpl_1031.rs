// Generated macro for impl_1031 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1031 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1031"}
// Dependencies: {}
impl < S > GraphQLType < S > for Intelligent where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Intelligent")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < i32 > > (arcstr :: literal ! ("iq") , i)] ; registry . build_interface_type :: < Self > (i , fields) . into_meta () } }
};
}
