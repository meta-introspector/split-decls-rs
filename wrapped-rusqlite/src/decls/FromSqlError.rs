macro_rules! deps {
    () => {
        Error!();
        FromSql!();
    };
}

macro_rules! FromSqlError {
    () => {
        deps!();
        # [doc = " Enum listing possible errors from [`FromSql`] trait."] # [derive (Debug)] # [non_exhaustive] pub enum FromSqlError { # [doc = " Error when an SQLite value is requested, but the type of the result"] # [doc = " cannot be converted to the requested Rust type."] InvalidType , # [doc = " Error when the i64 value returned by SQLite cannot be stored into the"] # [doc = " requested type."] OutOfRange (i64) , # [doc = " Error when the blob result returned by SQLite cannot be stored into the"] # [doc = " requested type due to a size mismatch."] InvalidBlobSize { # [doc = " The expected size of the blob."] expected_size : usize , # [doc = " The actual size of the blob that was returned."] blob_size : usize , } , # [doc = " An error case available for implementors of the [`FromSql`] trait."] Other (Box < dyn Error + Send + Sync + 'static >) , }
    };
}

FromSqlError!();