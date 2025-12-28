macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlResult!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl FromSql for std :: sync :: Arc < str > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (Into :: into) } }
    };
}

impl_356!()