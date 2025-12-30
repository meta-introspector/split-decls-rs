// Generated macro for ExecutionOutput (struct)
macro_rules! Depcrate_types_subscriptionsExecutionOutput {
() => {
// Module: crate::types::subscriptions
// Provides: {"ExecutionOutput"}
// Dependencies: {}
# [doc = " Represents the result of executing a GraphQL operation (after parsing and validating has been"] # [doc = " done)."] # [derive (Debug , Serialize)] pub struct ExecutionOutput < S > { # [doc = " The output data."] pub data : Value < S > , # [doc = " The errors that occurred. Note that the presence of errors does not mean there is no data."] # [doc = " The output can have both data and errors."] # [serde (bound (serialize = "S: ScalarValue"))] pub errors : Vec < ExecutionError < S > > , }
};
}
