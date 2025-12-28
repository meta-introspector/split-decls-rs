macro_rules! deps {
    () => {
        FromSqlResult!();
        FromSql!();
        ValueRef!();
    };
}

macro_rules! impl_360 {
    () => {
        deps!();
        impl FromSql for std :: sync :: Arc < [u8] > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (std :: sync :: Arc :: < [u8] > :: from) } }
    };
}

impl_360!();