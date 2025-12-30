// Generated macro for impl_592 (impl)
macro_rules! Depcrate_types_nullableimpl_592 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_592"}
// Dependencies: {}
impl < S , T > GraphQLValueAsync < S > for Nullable < T > where T : GraphQLValueAsync < S > , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , _ : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> crate :: BoxFuture < 'a , ExecutionResult < S > > { let f = async move { let value = match self { Self :: Some (obj) => executor . resolve_into_value_async (info , obj) . await , _ => Value :: null () , } ; Ok (value) } ; Box :: pin (f) } }
};
}
