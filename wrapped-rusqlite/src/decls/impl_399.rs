macro_rules! deps {
    () => {
        FromSql!();
        FromSqlError!();
        ValueRef!();
        FromSqlResult!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        # [doc = " YYYY-MM-DD HH:MM"] # [doc = " YYYY-MM-DDTHH:MM"] # [doc = " YYYY-MM-DD HH:MM:SS"] # [doc = " YYYY-MM-DDTHH:MM:SS"] # [doc = " YYYY-MM-DD HH:MM:SS.SSS"] # [doc = " YYYY-MM-DDTHH:MM:SS.SSS"] # [doc = " => ISO 8601 combined date and time with timezone"] impl FromSql for PrimitiveDateTime { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { Self :: parse (s , & PRIMITIVE_DATE_TIME_FORMAT) . map_err (| err | FromSqlError :: Other (err . into ())) }) } }
    };
}

impl_399!();