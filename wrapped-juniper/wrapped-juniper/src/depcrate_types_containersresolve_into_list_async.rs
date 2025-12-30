// Generated macro for resolve_into_list_async (function)
macro_rules! Depcrate_types_containersresolve_into_list_async {
() => {
// Module: crate::types::containers
// Provides: {"resolve_into_list_async"}
// Dependencies: {}
async fn resolve_into_list_async < 'a , 't , S , T , I > (executor : & 'a Executor < 'a , 'a , T :: Context , S > , info : & 'a T :: TypeInfo , items : I ,) -> ExecutionResult < S > where I : Iterator < Item = & 't T > + ExactSizeIterator , T : GraphQLValueAsync < S > + ? Sized + 't , T :: TypeInfo : Sync , T :: Context : Sync , S : ScalarValue + Send + Sync , { use futures :: stream :: { FuturesOrdered , StreamExt as _ } ; let stop_on_null = executor . current_type () . list_contents () . expect ("Current type is not a list type") . is_non_null () ; let mut futures = items . map (async | it | executor . resolve_into_value_async (info , it) . await) . collect :: < FuturesOrdered < _ > > () ; let mut values = Vec :: with_capacity (futures . len ()) ; while let Some (value) = futures . next () . await { if stop_on_null && value . is_null () { return Ok (value) ; } values . push (value) ; } Ok (Value :: list (values)) }
};
}
