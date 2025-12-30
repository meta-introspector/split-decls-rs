// Generated macro for impl_1047 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1047 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1047"}
// Dependencies: {}
impl < S > GraphQLValue < S > for QueryRoot where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } }
};
}
