macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlError!();
        FromSqlResult!();
        FromSql!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        # [doc = " \"HH:MM:SS.SSS\" => time."] impl FromSql for Time { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | s . parse () . map_err (FromSqlError :: other)) } }
    };
}

impl_372!();