macro_rules! deps {
    () => {
        ValueRef!();
        FromSql!();
        FromSqlResult!();
    };
}

macro_rules! impl_354 {
    () => {
        deps!();
        impl FromSql for Box < str > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (Into :: into) } }
    };
}

impl_354!();