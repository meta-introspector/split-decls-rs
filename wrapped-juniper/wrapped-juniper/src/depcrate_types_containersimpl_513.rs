// Generated macro for impl_513 (impl)
macro_rules! Depcrate_types_containersimpl_513 {
() => {
// Module: crate::types::containers
// Provides: {"impl_513"}
// Dependencies: {}
impl < S , T > GraphQLValue < S > for Option < T > where S : ScalarValue , T : GraphQLValue < S > , { type Context = T :: Context ; type TypeInfo = T :: TypeInfo ; fn type_name (& self , _ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn resolve (& self , info : & Self :: TypeInfo , _ : Option < & [Selection < S >] > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { match * self { Some (ref obj) => executor . resolve (info , obj) , None => Ok (Value :: null ()) , } } }
};
}
