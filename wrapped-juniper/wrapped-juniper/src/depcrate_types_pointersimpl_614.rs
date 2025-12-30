// Generated macro for impl_614 (impl)
macro_rules! Depcrate_types_pointersimpl_614 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_614"}
// Dependencies: {}
impl < S , T > GraphQLValueAsync < S > for Arc < T > where T : GraphQLValueAsync < S > + Send + ? Sized , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , selection_set : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> BoxFuture < 'a , ExecutionResult < S > > { (* * self) . resolve_async (info , selection_set , executor) } }
};
}
