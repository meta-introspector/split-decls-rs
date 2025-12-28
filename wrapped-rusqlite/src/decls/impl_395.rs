macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlResult!();
        FromSql!();
        FromSqlError!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [doc = " \"YYYY-MM-DD\" => ISO 8601 calendar date without timezone."] impl FromSql for Date { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { Self :: parse (s , & DATE_FORMAT) . map_err (| err | FromSqlError :: Other (err . into ())) }) } }
    };
}

impl_395!()