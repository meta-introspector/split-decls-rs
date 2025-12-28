macro_rules! deps {
    () => {
        ValueRef!();
        Null!();
        FromSql!();
        FromSqlResult!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl < T : FromSql > FromSql for Option < T > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Null => Ok (None) , _ => FromSql :: column_result (value) . map (Some) , } } }
    };
}

impl_364!();