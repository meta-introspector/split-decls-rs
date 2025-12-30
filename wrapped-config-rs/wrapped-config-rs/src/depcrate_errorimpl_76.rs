// Generated macro for impl_76 (impl)
macro_rules! Depcrate_errorimpl_76 {
() => {
// Module: crate::error
// Provides: {"impl_76"}
// Dependencies: {}
impl fmt :: Display for Unexpected { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> result :: Result < () , fmt :: Error > { match * self { Unexpected :: Bool (b) => write ! (f , "boolean `{b}`") , Unexpected :: I64 (i) => write ! (f , "64-bit integer `{i}`") , Unexpected :: I128 (i) => write ! (f , "128-bit integer `{i}`") , Unexpected :: U64 (i) => write ! (f , "64-bit unsigned integer `{i}`") , Unexpected :: U128 (i) => write ! (f , "128-bit unsigned integer `{i}`") , Unexpected :: Float (v) => write ! (f , "floating point `{v}`") , Unexpected :: Str (ref s) => write ! (f , "string {s:?}") , Unexpected :: Unit => write ! (f , "unit value") , Unexpected :: Seq => write ! (f , "sequence") , Unexpected :: Map => write ! (f , "map") , } } }
};
}
