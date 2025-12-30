// Generated macro for impl_22 (impl)
macro_rules! Depcrate_errorimpl_22 {
() => {
// Module: crate::error
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " The Display implementation allows the std::error::Error implementation"] impl < I : fmt :: Display > fmt :: Display for Error < I > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "error {:?} at: {}" , self . code , self . input) } }
};
}
