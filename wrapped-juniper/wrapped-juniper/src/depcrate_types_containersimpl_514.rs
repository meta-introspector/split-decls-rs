// Generated macro for impl_514 (impl)
macro_rules! Depcrate_types_containersimpl_514 {
() => {
// Module: crate::types::containers
// Provides: {"impl_514"}
// Dependencies: {}
impl < S , T > GraphQLValueAsync < S > for Option < T > where T : GraphQLValueAsync < S > , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , _ : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> crate :: BoxFuture < 'a , ExecutionResult < S > > { let f = async move { let value = match self { Some (obj) => executor . resolve_into_value_async (info , obj) . await , None => Value :: null () , } ; Ok (value) } ; Box :: pin (f) } }
};
}
