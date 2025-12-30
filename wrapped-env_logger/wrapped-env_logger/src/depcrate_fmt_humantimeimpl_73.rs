// Generated macro for impl_73 (impl)
macro_rules! Depcrate_fmt_humantimeimpl_73 {
() => {
// Module: crate::fmt::humantime
// Provides: {"impl_73"}
// Dependencies: {}
impl fmt :: Display for Timestamp { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let Ok (ts) = jiff :: Timestamp :: try_from (self . time) else { return Err (fmt :: Error) ; } ; match self . precision { TimestampPrecision :: Seconds => write ! (f , "{ts:.0}") , TimestampPrecision :: Millis => write ! (f , "{ts:.3}") , TimestampPrecision :: Micros => write ! (f , "{ts:.6}") , TimestampPrecision :: Nanos => write ! (f , "{ts:.9}") , } } }
};
}
