macro_rules! deps {
    () => {
        FromSqlError!();
        FromSql!();
        FromSqlResult!();
        ValueRef!();
    };
}

macro_rules! impl_308 {
    () => {
        deps!();
        # [doc = " \"YYYY-MM-DD\" => ISO 8601 calendar date without timezone."] impl FromSql for NaiveDate { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | Self :: parse_from_str (s , "%F") . map_err (FromSqlError :: other)) } }
    };
}

impl_308!()