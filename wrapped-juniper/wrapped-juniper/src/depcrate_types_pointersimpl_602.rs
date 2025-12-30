// Generated macro for impl_602 (impl)
macro_rules! Depcrate_types_pointersimpl_602 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_602"}
// Dependencies: {}
impl < S , T > GraphQLValueAsync < S > for Box < T > where T : GraphQLValueAsync < S > + ? Sized , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , selection_set : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> BoxFuture < 'a , ExecutionResult < S > > { (* * self) . resolve_async (info , selection_set , executor) } }
};
}
