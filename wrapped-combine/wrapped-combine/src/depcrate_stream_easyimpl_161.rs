// Generated macro for impl_161 (impl)
macro_rules! Depcrate_stream_easyimpl_161 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_161"}
// Dependencies: {}
impl < T : fmt :: Display , R : fmt :: Display > fmt :: Display for Info < T , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Info :: Token (ref c) => write ! (f , "`{}`" , c) , Info :: Range (ref c) => write ! (f , "`{}`" , c) , Info :: Owned (ref s) => write ! (f , "{}" , s) , Info :: Static (s) => write ! (f , "{}" , s) , } } }
};
}
