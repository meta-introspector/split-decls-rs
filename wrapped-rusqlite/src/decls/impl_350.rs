macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlError!();
        FromSqlResult!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl FromSql for f32 { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Integer (i) => Ok (i as Self) , ValueRef :: Real (f) => Ok (f as Self) , _ => Err (FromSqlError :: InvalidType) , } } }
    };
}

impl_350!();