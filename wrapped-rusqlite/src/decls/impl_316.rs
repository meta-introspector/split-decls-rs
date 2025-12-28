macro_rules! deps {
    () => {
        FromSql!();
        FromSqlResult!();
        ValueRef!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        # [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `DateTime<Utc>`."] impl FromSql for DateTime < Utc > { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { { let s = value . as_str () ? ; let fmt = if s . len () >= 11 && s . as_bytes () [10] == b'T' { "%FT%T%.f%#z" } else { "%F %T%.f%#z" } ; if let Ok (dt) = DateTime :: parse_from_str (s , fmt) { return Ok (dt . with_timezone (& Utc)) ; } } NaiveDateTime :: column_result (value) . map (| dt | Utc . from_utc_datetime (& dt)) } }
    };
}

impl_316!();