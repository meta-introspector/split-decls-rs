// Generated macro for impl_518 (impl)
macro_rules! Depcrate_types_containersimpl_518 {
() => {
// Module: crate::types::containers
// Provides: {"impl_518"}
// Dependencies: {}
impl < S , T > GraphQLValue < S > for Vec < T > where T : GraphQLValue < S > , S : ScalarValue , { type Context = T :: Context ; type TypeInfo = T :: TypeInfo ; fn type_name (& self , _ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn resolve (& self , info : & Self :: TypeInfo , _ : Option < & [Selection < S >] > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { resolve_into_list (executor , info , self . iter ()) } }
};
}
