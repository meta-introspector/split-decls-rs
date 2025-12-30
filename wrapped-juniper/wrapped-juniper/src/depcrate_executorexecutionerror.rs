// Generated macro for ExecutionError (struct)
macro_rules! Depcrate_executorExecutionError {
() => {
// Module: crate::executor
// Provides: {"ExecutionError"}
// Dependencies: {}
# [doc = " Error type for errors that occur during query execution"] # [doc = ""] # [doc = " All execution errors contain the source position in the query of the field"] # [doc = " that failed to resolve. It also contains the field stack."] # [derive (Clone , Debug , PartialEq)] pub struct ExecutionError < S > { location : SourcePosition , path : Vec < String > , error : FieldError < S > , }
};
}
