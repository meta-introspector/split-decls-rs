// Generated macro for impl_9 (impl)
macro_rules! Depcrate_errorsimpl_9 {
() => {
// Module: crate::errors
// Provides: {"impl_9"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> core :: result :: Result < () , fmt :: Error > { match self { Self :: Algorithm => write ! (f , "unsupported algorithm") , Self :: B64Encoding (err) => write ! (f , "{err}") , Self :: Crypto => write ! (f , "cryptographic error") , Self :: OutputSize { provided , expected } => match provided { Ordering :: Less => write ! (f , "output size too short, expected at least {expected} bytes" ,) , Ordering :: Equal => write ! (f , "output size unexpected, expected {expected} bytes") , Ordering :: Greater => { write ! (f , "output size too long, expected at most {expected} bytes") } } , Self :: ParamNameDuplicated => f . write_str ("duplicate parameter") , Self :: ParamNameInvalid => f . write_str ("invalid parameter name") , Self :: ParamValueInvalid (val_err) => write ! (f , "invalid parameter value: {val_err}") , Self :: ParamsMaxExceeded => f . write_str ("maximum number of parameters reached") , Self :: Password => write ! (f , "invalid password") , Self :: PhcStringField => write ! (f , "password hash string missing field") , Self :: PhcStringTrailingData => { write ! (f , "password hash string contains trailing characters") } Self :: SaltInvalid (val_err) => write ! (f , "salt invalid: {val_err}") , Self :: Version => write ! (f , "invalid algorithm version") , Self :: OutOfMemory => write ! (f , "out of memory") , } } }
};
}
