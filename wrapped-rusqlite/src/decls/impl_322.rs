macro_rules! deps {
    () => {
        Error!();
        FromSqlError!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl FromSqlError { # [doc = " Converts an arbitrary error type to [`FromSqlError`]."] # [doc = ""] # [doc = " This is a convenience function that boxes and unsizes the error type. It's main purpose is"] # [doc = " to be usable in the `map_err` method. So instead of"] # [doc = " `result.map_err(|error| FromSqlError::Other(Box::new(error))` you can write"] # [doc = " `result.map_err(FromSqlError::other)`."] pub fn other < E : Error + Send + Sync + 'static > (error : E) -> Self { Self :: Other (Box :: new (error)) } }
    };
}

impl_322!();