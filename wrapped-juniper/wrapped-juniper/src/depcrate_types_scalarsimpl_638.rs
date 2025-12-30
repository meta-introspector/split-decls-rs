// Generated macro for impl_638 (impl)
macro_rules! Depcrate_types_scalarsimpl_638 {
() => {
// Module: crate::types::scalars
// Provides: {"impl_638"}
// Dependencies: {}
impl < S > GraphQLValueAsync < S > for str where S : ScalarValue + Send + Sync , { fn resolve_async < 'a > (& 'a self , info : & 'a Self :: TypeInfo , selection_set : Option < & 'a [Selection < S >] > , executor : & 'a Executor < Self :: Context , S > ,) -> crate :: BoxFuture < 'a , ExecutionResult < S > > { use futures :: future ; Box :: pin (future :: ready (self . resolve (info , selection_set , executor))) } }
};
}
