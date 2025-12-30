// Generated macro for impl_613 (impl)
macro_rules! Depcrate_types_pointersimpl_613 {
() => {
// Module: crate::types::pointers
// Provides: {"impl_613"}
// Dependencies: {}
impl < S , T > GraphQLValue < S > for Arc < T > where S : ScalarValue , T : GraphQLValue < S > + ? Sized , { type Context = T :: Context ; type TypeInfo = T :: TypeInfo ; fn type_name (& self , info : & Self :: TypeInfo) -> Option < ArcStr > { (* * self) . type_name (info) } fn resolve_into_type (& self , info : & Self :: TypeInfo , name : & str , selection_set : Option < & [Selection < S >] > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { (* * self) . resolve_into_type (info , name , selection_set , executor) } fn resolve_field (& self , info : & Self :: TypeInfo , field : & str , args : & Arguments < S > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { (* * self) . resolve_field (info , field , args , executor) } fn resolve (& self , info : & Self :: TypeInfo , selection_set : Option < & [Selection < S >] > , executor : & Executor < Self :: Context , S > ,) -> ExecutionResult < S > { (* * self) . resolve (info , selection_set , executor) } }
};
}
