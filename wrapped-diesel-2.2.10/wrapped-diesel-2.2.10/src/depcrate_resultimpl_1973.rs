// Generated macro for impl_1973 (impl)
macro_rules! Depcrate_resultimpl_1973 {
() => {
// Module: crate::result
// Provides: {"impl_1973"}
// Dependencies: {}
impl StdError for ConnectionError { fn cause (& self) -> Option < & dyn StdError > { match * self { ConnectionError :: InvalidCString (ref e) => Some (e) , ConnectionError :: CouldntSetupConfiguration (ref e) => Some (e) , _ => None , } } }
};
}
