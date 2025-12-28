macro_rules! deps {
    () => {
        FromSql!();
        FromSqlResult!();
        ValueRef!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl FromSql for Box < [u8] > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (Box :: < [u8] > :: from) } }
    };
}

impl_358!()