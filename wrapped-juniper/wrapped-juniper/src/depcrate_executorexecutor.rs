// Generated macro for Executor (struct)
macro_rules! Depcrate_executorExecutor {
() => {
// Module: crate::executor
// Provides: {"Executor"}
// Dependencies: {}
# [doc = " Query execution engine"] # [doc = ""] # [doc = " The executor helps drive the query execution in a schema. It keeps track"] # [doc = " of the current field stack, context, variables, and errors."] pub struct Executor < 'r , 'a , CtxT , S = DefaultScalarValue > where CtxT : 'a , S : 'a , { fragments : & 'r HashMap < & 'a str , Fragment < 'a , S > > , variables : & 'r Variables < S > , current_selection_set : Option < & 'r [Selection < 'a , S >] > , parent_selection_set : Option < & 'r [Selection < 'a , S >] > , current_type : TypeType < 'a , S > , schema : & 'a SchemaType < S > , context : & 'a CtxT , errors : & 'r RwLock < Vec < ExecutionError < S > > > , field_path : Arc < FieldPath < 'a > > , }
};
}
