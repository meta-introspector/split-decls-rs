// Generated macro for resolve_selection_set_into_async (function)
macro_rules! Depcrate_types_async_awaitresolve_selection_set_into_async {
() => {
// Module: crate::types::async_await
// Provides: {"resolve_selection_set_into_async"}
// Dependencies: {}
fn resolve_selection_set_into_async < 'a , 'e , T , S > (instance : & 'a T , info : & 'a T :: TypeInfo , selection_set : & 'e [Selection < 'e , S >] , executor : & 'e Executor < 'e , 'e , T :: Context , S > ,) -> BoxFuture < 'a , Value < S > > where T : GraphQLValueAsync < S > + ? Sized , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , 'e : 'a , { Box :: pin (resolve_selection_set_into_async_recursive (instance , info , selection_set , executor ,)) }
};
}
