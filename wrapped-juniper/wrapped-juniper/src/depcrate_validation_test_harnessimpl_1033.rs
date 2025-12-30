// Generated macro for impl_1033 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1033 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1033"}
// Dependencies: {}
impl < S > GraphQLType < S > for Human where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Human")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < String > > (arcstr :: literal ! ("name") , i) . argument (registry . arg :: < Option < bool > > (arcstr :: literal ! ("surname") , i)) , registry . field :: < Option < Vec < Option < Pet > > > > (arcstr :: literal ! ("pets") , i) , registry . field :: < Option < Vec < Human > > > (arcstr :: literal ! ("relatives") , i) , registry . field :: < Option < i32 > > (arcstr :: literal ! ("iq") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < Being > (i) , registry . get_type :: < Intelligent > (i) ,]) . into_meta () } }
};
}
