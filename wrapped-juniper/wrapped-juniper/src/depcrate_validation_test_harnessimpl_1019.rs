// Generated macro for impl_1019 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1019 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1019"}
// Dependencies: {}
impl < S > GraphQLType < S > for DogCommand where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("DogCommand")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { registry . build_enum_type :: < Self > (i , & [EnumValue :: new ("SIT") , EnumValue :: new ("HEEL") , EnumValue :: new ("DOWN") ,] ,) . into_meta () } }
};
}
