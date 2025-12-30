// Generated macro for BatchToRequestMapper (type)
macro_rules! Depcrate_requestBatchToRequestMapper {
() => {
// Module: crate::request
// Provides: {"BatchToRequestMapper"}
// Dependencies: {}
type BatchToRequestMapper = fn (< < GraphQLBatchRequest as FromRequest > :: Future as Future > :: Output) -> Result < GraphQLRequest > ;
};
}
