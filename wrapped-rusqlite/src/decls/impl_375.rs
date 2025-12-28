macro_rules! deps {
    () => {
        ToSqlOutput!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        # [doc = " UTC time => UTC RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DDTHH:MM:SS.SSSZ\")."] impl ToSql for Timestamp { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . to_string ())) } }
    };
}

impl_375!();