macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlResult!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl FromSql for bool { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { i64 :: column_result (value) . map (| i | i != 0) } }
    };
}

impl_352!();