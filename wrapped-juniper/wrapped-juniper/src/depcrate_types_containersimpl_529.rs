// Generated macro for impl_529 (impl)
macro_rules! Depcrate_types_containersimpl_529 {
() => {
// Module: crate::types::containers
// Provides: {"impl_529"}
// Dependencies: {}
impl < S , T , const N : usize > GraphQLValue < S > for [T ; N] where S : ScalarValue , T : GraphQLValue < S > , { type Context = T :: Context ; type TypeInfo = T :: TypeInfo ; fn type_name (& self , _ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn resolve (& self , info : & Self :: TypeInfo , _ : Option < & [Selection < S >] > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { resolve_into_list (executor , info , self . iter ()) } }
};
}
