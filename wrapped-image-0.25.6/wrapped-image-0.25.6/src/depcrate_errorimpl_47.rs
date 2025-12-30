// Generated macro for impl_47 (impl)
macro_rules! Depcrate_errorimpl_47 {
() => {
// Module: crate::error
// Provides: {"impl_47"}
// Dependencies: {}
impl Error for EncodingError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . underlying { None => None , Some (source) => Some (& * * source) , } } }
};
}
