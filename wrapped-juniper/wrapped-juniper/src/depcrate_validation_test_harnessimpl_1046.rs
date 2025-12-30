// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1046 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1046"}
// Dependencies: {}
impl < S > GraphQLType < S > for QueryRoot where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("QueryRoot")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = & [registry . field :: < Option < Human > > (arcstr :: literal ! ("human") , i) . argument (registry . arg :: < Option < ID > > (arcstr :: literal ! ("id") , i)) , registry . field :: < Option < Alien > > (arcstr :: literal ! ("alien") , i) , registry . field :: < Option < Dog > > (arcstr :: literal ! ("dog") , i) , registry . field :: < Option < Cat > > (arcstr :: literal ! ("cat") , i) , registry . field :: < Option < Pet > > (arcstr :: literal ! ("pet") , i) , registry . field :: < Option < CatOrDog > > (arcstr :: literal ! ("catOrDog") , i) , registry . field :: < Option < DogOrHuman > > (arcstr :: literal ! ("dorOrHuman") , i) , registry . field :: < Option < HumanOrAlien > > (arcstr :: literal ! ("humanOrAlien") , i) , registry . field :: < Option < ComplicatedArgs > > (arcstr :: literal ! ("complicatedArgs") , i) ,] ; registry . build_object_type :: < Self > (i , fields) . into_meta () } }
};
}
