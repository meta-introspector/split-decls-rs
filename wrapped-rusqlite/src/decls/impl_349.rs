macro_rules! deps {
    () => {
        FromSqlResult!();
        FromSql!();
        ValueRef!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl FromSql for i64 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_i64 () } }
    };
}

impl_349!();