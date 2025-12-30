// Generated macro for impl_351 (impl)
macro_rules! Depcrate_nostd_ioimpl_351 {
() => {
// Module: crate::nostd_io
// Provides: {"impl_351"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . repr { Repr :: Custom (ref c) => c . error . fmt (fmt) , Repr :: Simple (kind) => write ! (fmt , "{}" , kind . as_str ()) , } } }
};
}
