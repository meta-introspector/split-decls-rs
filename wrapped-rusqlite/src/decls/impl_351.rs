macro_rules! deps {
    () => {
        FromSql!();
        FromSqlError!();
        FromSqlResult!();
        ValueRef!();
    };
}

macro_rules! impl_351 {
    () => {
        deps!();
        impl FromSql for f64 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Integer (i) => Ok (i as Self) , ValueRef :: Real (f) => Ok (f) , _ => Err (FromSqlError :: InvalidType) , } } }
    };
}

impl_351!()