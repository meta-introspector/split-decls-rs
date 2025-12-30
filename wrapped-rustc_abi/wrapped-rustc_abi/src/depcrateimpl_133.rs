// Generated macro for impl_133 (impl)
macro_rules! Depcrateimpl_133 {
() => {
// Module: crate
// Provides: {"impl_133"}
// Dependencies: {}
impl fmt :: Debug for WrappingRange { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start > self . end { write ! (fmt , "(..={}) | ({}..)" , self . end , self . start) ? ; } else { write ! (fmt , "{}..={}" , self . start , self . end) ? ; } Ok (()) } }
};
}
