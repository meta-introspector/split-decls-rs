macro_rules! deps {
    () => {
        FromSqlError!();
        Error!();
        Blob!();
        Type!();
        Null!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        # [doc = " The conversion isn't precise, but it's convenient to have it"] # [doc = " to allow use of `get_raw(…).as_…()?` in callbacks that take `Error`."] impl From < FromSqlError > for Error { # [cold] fn from (err : FromSqlError) -> Self { match err { FromSqlError :: OutOfRange (val) => Self :: IntegralValueOutOfRange (UNKNOWN_COLUMN , val) , FromSqlError :: InvalidBlobSize { .. } => { Self :: FromSqlConversionFailure (UNKNOWN_COLUMN , Type :: Blob , Box :: new (err)) } FromSqlError :: Other (source) => { Self :: FromSqlConversionFailure (UNKNOWN_COLUMN , Type :: Null , source) } _ => Self :: FromSqlConversionFailure (UNKNOWN_COLUMN , Type :: Null , Box :: new (err)) , } } }
    };
}

impl_5!();