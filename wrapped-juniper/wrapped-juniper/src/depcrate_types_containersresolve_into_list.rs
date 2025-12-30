// Generated macro for resolve_into_list (function)
macro_rules! Depcrate_types_containersresolve_into_list {
() => {
// Module: crate::types::containers
// Provides: {"resolve_into_list"}
// Dependencies: {}
fn resolve_into_list < 't , S , T , I > (executor : & Executor < T :: Context , S > , info : & T :: TypeInfo , iter : I ,) -> ExecutionResult < S > where S : ScalarValue , I : Iterator < Item = & 't T > + ExactSizeIterator , T : GraphQLValue < S > + ? Sized + 't , { let stop_on_null = executor . current_type () . list_contents () . expect ("Current type is not a list type") . is_non_null () ; let mut result = Vec :: with_capacity (iter . len ()) ; for o in iter { let val = executor . resolve (info , o) ? ; if stop_on_null && val . is_null () { return Ok (val) ; } else { result . push (val) } } Ok (Value :: list (result)) }
};
}
