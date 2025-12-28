macro_rules! deps {
    () => {
        FromSqlResult!();
        Value!();
        ValueRef!();
        FromSql!();
    };
}

macro_rules! impl_366 {
    () => {
        deps!();
        impl FromSql for Value { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { Ok (value . into ()) } }
    };
}

impl_366!();