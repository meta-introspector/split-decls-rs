// Generated macro for impl_142 (impl)
macro_rules! Depcrateimpl_142 {
() => {
// Module: crate
// Provides: {"impl_142"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let ErrorKind :: StackExhausted = self . kind { write ! (f , "Error parsing regex: {}" , self . kind) } else { write ! (f , "Error parsing regex near '{}' at character offset {}: {}" , self . surround , self . pos , self . kind) } } }
};
}
