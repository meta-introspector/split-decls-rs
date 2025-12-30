// Generated macro for impl_359 (impl)
macro_rules! Depcrate_traitsimpl_359 {
() => {
// Module: crate::traits
// Provides: {"impl_359"}
// Dependencies: {}
impl fmt :: Display for DecodeError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Empty => write ! (f , "empty value provided") , Self :: InvalidDigit => { write ! (f , "invalid digit character") } Self :: InputSize => write ! (f , "input size is too small to fit in the given precision") , Self :: Precision => write ! (f , "the deserialized number is larger than the given precision") , } } }
};
}
