// Generated macro for impl_519 (impl)
macro_rules! Depcrate_types_containersimpl_519 {
() => {
// Module: crate::types::containers
// Provides: {"impl_519"}
// Dependencies: {}
impl < S , T > GraphQLValueAsync < S > for Vec < T > where T : GraphQLValueAsync < S > , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , _ : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> crate :: BoxFuture < 'a , ExecutionResult < S > > { let f = resolve_into_list_async (executor , info , self . iter ()) ; Box :: pin (f) } }
};
}
