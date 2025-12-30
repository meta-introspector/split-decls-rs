// Generated macro for impl_530 (impl)
macro_rules! Depcrate_types_containersimpl_530 {
() => {
// Module: crate::types::containers
// Provides: {"impl_530"}
// Dependencies: {}
impl < S , T , const N : usize > GraphQLValueAsync < S > for [T ; N] where T : GraphQLValueAsync < S > , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , _ : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> crate :: BoxFuture < 'a , ExecutionResult < S > > { let f = resolve_into_list_async (executor , info , self . iter ()) ; Box :: pin (f) } }
};
}
