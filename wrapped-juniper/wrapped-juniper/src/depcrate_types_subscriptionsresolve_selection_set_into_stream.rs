// Generated macro for resolve_selection_set_into_stream (function)
macro_rules! Depcrate_types_subscriptionsresolve_selection_set_into_stream {
() => {
// Module: crate::types::subscriptions
// Provides: {"resolve_selection_set_into_stream"}
// Dependencies: {}
# [doc = " Wrapper function around `resolve_selection_set_into_stream_recursive`."] # [doc = " This wrapper is necessary because async fns can not be recursive."] # [doc = " Panics if executor's current selection set is None."] pub (crate) fn resolve_selection_set_into_stream < 'i , 'inf , 'ref_e , 'e , 'res , 'fut , T , S > (instance : & 'i T , info : & 'inf T :: TypeInfo , executor : & 'ref_e Executor < 'ref_e , 'e , T :: Context , S > ,) -> BoxFuture < 'fut , Value < ValuesStream < 'res , S > > > where 'inf : 'res , 'e : 'res , 'i : 'fut , 'e : 'fut , 'ref_e : 'fut , 'res : 'fut , T : GraphQLSubscriptionValue < S > + ? Sized , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { Box :: pin (resolve_selection_set_into_stream_recursive (instance , info , executor ,)) }
};
}
