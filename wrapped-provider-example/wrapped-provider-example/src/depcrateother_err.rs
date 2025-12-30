// Generated macro for other_err (function)
macro_rules! Depcrateother_err {
() => {
// Module: crate
// Provides: {"other_err"}
// Dependencies: {}
# [doc = " Since hpke-rs does not implement `core::error::Error` for `no_std`, we fall back to"] # [doc = " using a string representation of the error."] # [cfg (not (feature = "std"))] fn other_err (error : impl fmt :: Display + Send + Sync + 'static) -> Error { struct DisplayError < T : fmt :: Display > (T) ; impl < T : fmt :: Display > StdError for DisplayError < T > { } impl < T : fmt :: Display > fmt :: Display for DisplayError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . 0) } } impl < T : fmt :: Display > fmt :: Debug for DisplayError < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } } Error :: Other (OtherError :: new (DisplayError (error))) }
};
}
