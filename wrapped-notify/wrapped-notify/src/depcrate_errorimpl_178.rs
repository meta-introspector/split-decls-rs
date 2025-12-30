// Generated macro for impl_178 (impl)
macro_rules! Depcrate_errorimpl_178 {
() => {
// Module: crate::error
// Provides: {"impl_178"}
// Dependencies: {}
impl StdError for Error { fn cause (& self) -> Option < & dyn StdError > { match self . kind { ErrorKind :: Io (ref cause) => Some (cause) , _ => None , } } }
};
}
