// Generated macro for impl_1374 (impl)
macro_rules! Depcrate_tests_type_info_testsimpl_1374 {
() => {
// Module: crate::tests::type_info_tests
// Provides: {"impl_1374"}
// Dependencies: {}
impl < S > GraphQLValue < S > for Node where S : ScalarValue , { type Context = () ; type TypeInfo = NodeTypeInfo ; fn type_name < 'i > (& self , info : & 'i Self :: TypeInfo) -> Option < ArcStr > { < Self as GraphQLType < S > > :: name (info) } fn resolve_field (& self , _ : & Self :: TypeInfo , field_name : & str , _ : & Arguments < S > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { executor . resolve (& () , & self . attributes . get (field_name) . unwrap ()) } }
};
}
