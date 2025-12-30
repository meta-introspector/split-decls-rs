// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1028 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1028"}
// Dependencies: {}
impl < S > GraphQLValue < S > for Cat where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } }
};
}
