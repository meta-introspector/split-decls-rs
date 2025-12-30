// Generated macro for impl_32 (impl)
macro_rules! Depcrate_durationimpl_32 {
() => {
// Module: crate::duration
// Provides: {"impl_32"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: InvalidCharacter (offset) => write ! (f , "invalid character at {}" , offset) , Error :: NumberExpected (offset) => write ! (f , "expected number at {}" , offset) , Error :: UnknownUnit { unit , value , .. } if unit . is_empty () => { write ! (f , "time unit needed, for example {0}sec or {0}ms" , value) } Error :: UnknownUnit { unit , .. } => { write ! (f , "unknown time unit {:?}, \
                    supported units: ns, us/µs, ms, sec, min, hours, days, \
                    weeks, months, years (and few variations)" , unit) } Error :: NumberOverflow => write ! (f , "number is too large or cannot be represented without a lack of precision (values below 1ns are not supported)") , Error :: Empty => write ! (f , "value was empty") , } } }
};
}
