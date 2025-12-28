macro_rules! deps {
    () => {
        ValueRef!();
        FromSql!();
        FromSqlResult!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        # [cfg (feature = "uuid")] impl FromSql for uuid :: Uuid { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let bytes = < [u8 ; 16] > :: column_result (value) ? ; Ok (Self :: from_u128 (u128 :: from_be_bytes (bytes))) } }
    };
}

impl_363!();