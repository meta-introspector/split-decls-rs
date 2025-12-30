// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1027 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1027"}
// Dependencies: {}
impl < S > GraphQLType < S > for Cat where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Cat")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < String > > (arcstr :: literal ! ("name") , i) . argument (registry . arg :: < Option < bool > > (arcstr :: literal ! ("surname") , i)) , registry . field :: < Option < String > > (arcstr :: literal ! ("nickname") , i) , registry . field :: < Option < bool > > (arcstr :: literal ! ("meows") , i) , registry . field :: < Option < i32 > > (arcstr :: literal ! ("meowVolume") , i) , registry . field :: < Option < FurColor > > (arcstr :: literal ! ("furColor") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < Being > (i) , registry . get_type :: < Pet > (i)]) . into_meta () } }
};
}
