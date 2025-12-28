macro_rules! deps {
    () => {
        ValueRef!();
        FromSql!();
        FromSqlResult!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        # [cfg (feature = "i128_blob")] impl FromSql for i128 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let bytes = < [u8 ; 16] > :: column_result (value) ? ; Ok (Self :: from_be_bytes (bytes) ^ (1_i128 << 127)) } }
    };
}

impl_362!()