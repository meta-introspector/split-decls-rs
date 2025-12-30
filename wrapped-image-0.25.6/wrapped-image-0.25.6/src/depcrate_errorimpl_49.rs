// Generated macro for impl_49 (impl)
macro_rules! Depcrate_errorimpl_49 {
() => {
// Module: crate::error
// Provides: {"impl_49"}
// Dependencies: {}
impl Error for DecodingError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . underlying { None => None , Some (source) => Some (& * * source) , } } }
};
}
