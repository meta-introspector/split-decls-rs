// Generated macro for impl_1041 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1041 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1041"}
// Dependencies: {}
impl < S > GraphQLType < S > for ComplexInput where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("ComplexInput")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . arg :: < bool > (arcstr :: literal ! ("requiredField") , i) , registry . arg :: < Option < i32 > > (arcstr :: literal ! ("intField") , i) , registry . arg :: < Option < String > > (arcstr :: literal ! ("stringField") , i) , registry . arg :: < Option < bool > > (arcstr :: literal ! ("booleanField") , i) , registry . arg :: < Option < Vec < Option < String > > > > (arcstr :: literal ! ("stringListField") , i) ,] ; registry . build_input_object_type :: < Self > (i , fields) . into_meta () } }
};
}
