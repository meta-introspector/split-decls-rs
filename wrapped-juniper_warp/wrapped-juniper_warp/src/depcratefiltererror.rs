// Generated macro for FilterError (enum)
macro_rules! DepcrateFilterError {
() => {
// Module: crate
// Provides: {"FilterError"}
// Dependencies: {}
# [doc = " Possible errors happening in [`Filter`]s during [`GraphQLBatchRequest`] extraction."] # [derive (Debug , Display)] enum FilterError { # [doc = " GET HTTP request misses query parameters."] # [display ("Missing GraphQL `query` string in query parameters")] MissingPathQuery , # [doc = " GET HTTP request contains ivalid `path` query parameter."] # [display ("Failed to deserialize GraphQL `variables` from JSON: {_0}")] InvalidPathVariables (serde_json :: Error) , # [doc = " POST HTTP request contains non-UTF-8 body."] # [display ("Request body is not a valid UTF-8 string: {_0}")] NonUtf8Body (str :: Utf8Error) , }
};
}
