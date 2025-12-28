macro_rules! deps {
    () => {
        FromSqlError!();
        FromSqlResult!();
        FromSql!();
        ValueRef!();
    };
}

macro_rules! impl_397 {
    () => {
        deps!();
        # [doc = " \"HH:MM\"/\"HH:MM:SS\"/\"HH:MM:SS.SSS\" => ISO 8601 time without timezone."] impl FromSql for Time { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { Self :: parse (s , & TIME_FORMAT) . map_err (| err | FromSqlError :: Other (err . into ())) }) } }
    };
}

impl_397!()