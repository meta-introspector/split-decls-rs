macro_rules! deps {
    () => {
        FromSqlResult!();
        ValueRef!();
        FromSqlError!();
        FromSql!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl < const N : usize > FromSql for [u8 ; N] { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let slice = value . as_blob () ? ; slice . try_into () . map_err (| _ | FromSqlError :: InvalidBlobSize { expected_size : N , blob_size : slice . len () , }) } }
    };
}

impl_361!();