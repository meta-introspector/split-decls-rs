// Generated macro for impl_1024 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1024 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1024"}
// Dependencies: {}
impl < S > GraphQLType < S > for FurColor where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("FurColor")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { registry . build_enum_type :: < Self > (i , & [EnumValue :: new ("BROWN") , EnumValue :: new ("BLACK") , EnumValue :: new ("TAN") , EnumValue :: new ("SPOTTED") ,] ,) . into_meta () } }
};
}
