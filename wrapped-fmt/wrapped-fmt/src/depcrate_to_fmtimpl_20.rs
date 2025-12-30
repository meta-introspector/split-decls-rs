// Generated macro for impl_20 (impl)
macro_rules! Depcrate_to_fmtimpl_20 {
() => {
// Module: crate::to_fmt
// Provides: {"impl_20"}
// Dependencies: {}
impl < V : sval :: Value > fmt :: Display for ToFmt < V > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match stream_to_fmt (f , & self . 0) { Ok (()) => Ok (()) , Err (e) => write ! (f , "<{}>" , e) , } } }
};
}
