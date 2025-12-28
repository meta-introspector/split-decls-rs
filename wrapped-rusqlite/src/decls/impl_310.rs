macro_rules! deps {
    () => {
        FromSqlResult!();
        ValueRef!();
        FromSqlError!();
        FromSql!();
    };
}

macro_rules! impl_310 {
    () => {
        deps!();
        # [doc = " \"HH:MM\"/\"HH:MM:SS\"/\"HH:MM:SS.SSS\" => ISO 8601 time without timezone."] impl FromSql for NaiveTime { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | { let fmt = match s . len () { 5 => "%H:%M" , 8 => "%T" , _ => "%T%.f" , } ; Self :: parse_from_str (s , fmt) . map_err (FromSqlError :: other) }) } }
    };
}

impl_310!();