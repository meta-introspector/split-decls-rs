// Generated macro for impl_45 (impl)
macro_rules! Depcrate_errorimpl_45 {
() => {
// Module: crate::error
// Provides: {"impl_45"}
// Dependencies: {}
impl Error for ParameterError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . underlying { None => None , Some (source) => Some (& * * source) , } } }
};
}
