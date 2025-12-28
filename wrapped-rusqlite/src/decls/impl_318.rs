macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlError!();
        FromSqlResult!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        # [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `DateTime<FixedOffset>`."] impl FromSql for DateTime < FixedOffset > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let s = String :: column_result (value) ? ; Self :: parse_from_rfc3339 (s . as_str ()) . or_else (| _ | Self :: parse_from_str (s . as_str () , "%F %T%.f%:z")) . map_err (FromSqlError :: other) } }
    };
}

impl_318!()