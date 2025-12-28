macro_rules! deps {
    () => {
        FromSqlResult!();
        ValueRef!();
        FromSql!();
    };
}

macro_rules! impl_353 {
    () => {
        deps!();
        impl FromSql for String { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (ToString :: to_string) } }
    };
}

impl_353!();