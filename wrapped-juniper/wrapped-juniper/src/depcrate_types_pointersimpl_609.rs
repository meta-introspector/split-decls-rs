// Generated macro for impl_609 (impl)
macro_rules! Depcrate_types_pointersimpl_609 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_609"}
// Dependencies: {}
impl < S , T > GraphQLValueAsync < S > for & T where T : GraphQLValueAsync < S > + ? Sized , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_field_async < 'b > (& 'b self , info : & 'b Self :: TypeInfo , field_name : & 'b str , arguments : & 'b Arguments < S > , executor : & 'b Executor < Self :: Context , S > ,) -> BoxFuture < 'b , ExecutionResult < S > > { (* * self) . resolve_field_async (info , field_name , arguments , executor) } fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , selection_set : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> BoxFuture < 'a , ExecutionResult < S > > { (* * self) . resolve_async (info , selection_set , executor) } }
};
}
