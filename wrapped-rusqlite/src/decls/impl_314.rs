macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        ToSqlOutput!();
    };
}

macro_rules! impl_314 {
    () => {
        deps!();
        # [doc = " Local time => UTC RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DD HH:MM:SS.SSS+00:00\")."] impl ToSql for DateTime < Local > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . with_timezone (& Utc) . format ("%F %T%.f%:z") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_314!();