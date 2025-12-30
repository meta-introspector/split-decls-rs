// Generated macro for impl_1040 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1040 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1040"}
// Dependencies: {}
impl < S > GraphQLValue < S > for HumanOrAlien where S : ScalarValue , { type Context = () ; type TypeInfo = () ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType > :: name (info) } }
};
}
