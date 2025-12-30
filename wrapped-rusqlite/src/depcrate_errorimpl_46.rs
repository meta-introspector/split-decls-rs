// Generated macro for impl_46 (impl)
macro_rules! Depcrate_errorimpl_46 {
() => {
// Module: crate::error
// Provides: {"impl_46"}
// Dependencies: {}
# [doc = " The conversion isn't precise, but it's convenient to have it"] # [doc = " to allow use of `get_raw(…).as_…()?` in callbacks that take `Error`."] impl From < FromSqlError > for Error { # [cold] fn from (err : FromSqlError) -> Self { match err { FromSqlError :: OutOfRange (val) => Self :: IntegralValueOutOfRange (UNKNOWN_COLUMN , val) , FromSqlError :: InvalidBlobSize { .. } => { Self :: FromSqlConversionFailure (UNKNOWN_COLUMN , Type :: Blob , Box :: new (err)) } FromSqlError :: Other (source) => { Self :: FromSqlConversionFailure (UNKNOWN_COLUMN , Type :: Null , source) } _ => Self :: FromSqlConversionFailure (UNKNOWN_COLUMN , Type :: Null , Box :: new (err)) , } } }
};
}
