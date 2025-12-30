// Generated macro for impl_1965 (impl)
macro_rules! Depcrate_resultimpl_1965 {
() => {
// Module: crate::result
// Provides: {"impl_1965"}
// Dependencies: {}
impl < T > OptionalExtension < T > for QueryResult < T > { fn optional (self) -> Result < Option < T > , Error > { match self { Ok (value) => Ok (Some (value)) , Err (Error :: NotFound) => Ok (None) , Err (e) => Err (e) , } } }
};
}
