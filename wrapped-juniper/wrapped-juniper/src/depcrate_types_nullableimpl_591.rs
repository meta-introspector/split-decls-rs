// Generated macro for impl_591 (impl)
macro_rules! Depcrate_types_nullableimpl_591 {
() => {
// Module: crate::types::nullable
// Provides: {"impl_591"}
// Dependencies: {}
impl < S , T > GraphQLValue < S > for Nullable < T > where S : ScalarValue , T : GraphQLValue < S > , { type Context = T :: Context ; type TypeInfo = T :: TypeInfo ; fn type_name (& self , _ : & Self :: TypeInfo) -> Option < ArcStr > { None } fn resolve (& self , info : & Self :: TypeInfo , _ : Option < & [Selection < S >] > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { match * self { Self :: Some (ref obj) => executor . resolve (info , obj) , _ => Ok (Value :: null ()) , } } }
};
}
