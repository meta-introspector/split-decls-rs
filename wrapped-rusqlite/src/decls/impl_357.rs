macro_rules! deps {
    () => {
        FromSql!();
        FromSqlResult!();
        ValueRef!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl FromSql for Vec < u8 > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (< [u8] > :: to_vec) } }
    };
}

impl_357!();