macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlResult!();
        FromSqlError!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        # [doc = " \"YYYY-MM-DD HH:MM:SS\"/\"YYYY-MM-DD HH:MM:SS.SSS\" => ISO 8601 combined date"] # [doc = " and time without timezone. (\"YYYY-MM-DDTHH:MM:SS\"/\"YYYY-MM-DDTHH:MM:SS.SSS\""] # [doc = " also supported)"] impl FromSql for NaiveDateTime { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { let fmt = if s . len () >= 11 && s . as_bytes () [10] == b'T' { "%FT%T%.f" } else { "%F %T%.f" } ; Self :: parse_from_str (s , fmt) . map_err (FromSqlError :: other) }) } }
    };
}

impl_312!();