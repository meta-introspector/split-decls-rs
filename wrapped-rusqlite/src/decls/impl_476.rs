macro_rules! deps {
    () => {
        FromSql!();
        FromSqlResult!();
        ValueRef!();
        FromSqlError!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        # [doc = " Deserialize text to `Url`."] impl FromSql for Url { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { match value { ValueRef :: Text (s) => { let s = std :: str :: from_utf8 (s) . map_err (FromSqlError :: other) ? ; Self :: parse (s) . map_err (FromSqlError :: other) } _ => Err (FromSqlError :: InvalidType) , } } }
    };
}

impl_476!();