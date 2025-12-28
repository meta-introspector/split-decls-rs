macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlResult!();
        FromSql!();
    };
}

macro_rules! impl_317 {
    () => {
        deps!();
        # [doc = " RFC3339 (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\") into `DateTime<Local>`."] impl FromSql for DateTime < Local > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { let utc_dt = DateTime :: < Utc > :: column_result (value) ? ; Ok (utc_dt . with_timezone (& Local)) } }
    };
}

impl_317!();