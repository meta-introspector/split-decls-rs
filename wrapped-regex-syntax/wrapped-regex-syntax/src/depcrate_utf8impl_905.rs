// Generated macro for impl_905 (impl)
macro_rules! Depcrate_utf8impl_905 {
() => {
// Module: crate::utf8
// Provides: {"impl_905"}
// Dependencies: {}
impl fmt :: Debug for Utf8Range { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start == self . end { write ! (f , "[{:X}]" , self . start) } else { write ! (f , "[{:X}-{:X}]" , self . start , self . end) } } }
};
}
