// Generated macro for impl_1022 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1022 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1022"}
// Dependencies: {}
impl < S > GraphQLType < S > for Dog where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("Dog")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < String > > (arcstr :: literal ! ("name") , i) . argument (registry . arg :: < Option < bool > > (arcstr :: literal ! ("surname") , i)) , registry . field :: < Option < String > > (arcstr :: literal ! ("nickname") , i) , registry . field :: < Option < i32 > > (arcstr :: literal ! ("barkVolume") , i) , registry . field :: < Option < bool > > (arcstr :: literal ! ("barks") , i) , registry . field :: < Option < bool > > (arcstr :: literal ! ("doesKnowCommand") , i) . argument (registry . arg :: < Option < DogCommand > > (arcstr :: literal ! ("dogCommand") , i)) , registry . field :: < Option < bool > > (arcstr :: literal ! ("isHousetrained") , i) . argument (registry . arg_with_default (arcstr :: literal ! ("atOtherHomes") , & true , i)) , registry . field :: < Option < bool > > (arcstr :: literal ! ("isAtLocation") , i) . argument (registry . arg :: < Option < i32 > > (arcstr :: literal ! ("x") , i)) . argument (registry . arg :: < Option < i32 > > (arcstr :: literal ! ("y") , i)) ,] ; registry . build_object_type :: < Self > (i , fields) . interfaces (& [registry . get_type :: < Being > (i) , registry . get_type :: < Pet > (i) , registry . get_type :: < Canine > (i) ,]) . into_meta () } }
};
}
