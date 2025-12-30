// Generated macro for impl_288 (impl)
macro_rules! Depcrate_utf8impl_288 {
() => {
// Module: crate::utf8
// Provides: {"impl_288"}
// Dependencies: {}
impl fmt :: Display for Utf8Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "invalid UTF-8 found at byte offset {}" , self . valid_up_to) } }
};
}
