// Generated macro for impl_50 (impl)
macro_rules! Depcrate_errorimpl_50 {
() => {
// Module: crate::error
// Provides: {"impl_50"}
// Dependencies: {}
impl fmt :: Display for LimitError { fn fmt (& self , fmt : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { match self . kind { LimitErrorKind :: InsufficientMemory => write ! (fmt , "Memory limit exceeded") , LimitErrorKind :: DimensionError => write ! (fmt , "Image size exceeds limit") , LimitErrorKind :: Unsupported { .. } => { write ! (fmt , "The following strict limits are specified but not supported by the opertation: ") ? ; Ok (()) } } } }
};
}
