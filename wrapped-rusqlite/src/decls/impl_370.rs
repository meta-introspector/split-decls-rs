macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlResult!();
        FromSqlError!();
        FromSql!();
    };
}

macro_rules! impl_370 {
    () => {
        deps!();
        # [doc = " \"YYYY-MM-DD\" => Gregorian calendar date."] impl FromSql for Date { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | s . parse () . map_err (FromSqlError :: other)) } }
    };
}

impl_370!();