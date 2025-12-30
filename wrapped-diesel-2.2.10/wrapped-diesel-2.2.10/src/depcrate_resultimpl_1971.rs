// Generated macro for impl_1971 (impl)
macro_rules! Depcrate_resultimpl_1971 {
() => {
// Module: crate::result
// Provides: {"impl_1971"}
// Dependencies: {}
impl StdError for Error { fn cause (& self) -> Option < & dyn StdError > { match * self { Error :: InvalidCString (ref e) => Some (e) , Error :: QueryBuilderError (ref e) => Some (& * * e) , Error :: DeserializationError (ref e) => Some (& * * e) , Error :: SerializationError (ref e) => Some (& * * e) , _ => None , } } }
};
}
