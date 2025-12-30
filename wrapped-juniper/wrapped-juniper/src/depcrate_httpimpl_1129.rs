// Generated macro for impl_1129 (impl)
macro_rules! Depcrate_httpimpl_1129 {
() => {
// Module: crate::http
// Provides: {"impl_1129"}
// Dependencies: {}
impl < S > GraphQLResponse < S > where S : ScalarValue , { # [doc = " Constructs a new [`GraphQLResponse`] from the provided execution [`Result`]."] # [must_use] pub fn from_result (r : Result < (Value < S > , Vec < ExecutionError < S > >) , GraphQLError >) -> Self { Self (r) } # [doc = " Unwraps this [`GraphQLResponse`] into its underlying execution [`Result`]."] pub fn into_result (self) -> Result < (Value < S > , Vec < ExecutionError < S > >) , GraphQLError > { self . 0 } # [doc = " Constructs an error [`GraphQLResponse`] outside the normal execution flow."] # [must_use] pub fn error (error : FieldError < S >) -> Self { Self (Ok ((Value :: null () , vec ! [ExecutionError :: at_origin (error)]))) } # [doc = " Indicates whether this [`GraphQLResponse`] contains a successful execution [`Result`]."] # [doc = ""] # [doc = " **NOTE**: There still might be errors in the response even though it's considered OK."] # [doc = "           This is by design in GraphQL."] # [must_use] pub fn is_ok (& self) -> bool { self . 0 . is_ok () } }
};
}
