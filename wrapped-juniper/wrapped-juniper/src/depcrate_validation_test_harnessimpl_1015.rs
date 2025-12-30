// Generated macro for impl_1015 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1015 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1015"}
// Dependencies: {}
impl < S > GraphQLType < S > for Canine where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Canine")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < String > > (arcstr :: literal ! ("name") , i) . argument (registry . arg :: < Option < bool > > (arcstr :: literal ! ("surname") , i))] ; registry . build_interface_type :: < Self > (i , fields) . into_meta () } }
};
}
