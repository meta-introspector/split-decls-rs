macro_rules! deps {
    () => {
        FromSqlResult!();
        FromSqlError!();
        ValueRef!();
        FromSql!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        # [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `Timestamp`."] impl FromSql for Timestamp { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () ? . parse :: < Timestamp > () . map_err (FromSqlError :: other) } }
    };
}

impl_376!()