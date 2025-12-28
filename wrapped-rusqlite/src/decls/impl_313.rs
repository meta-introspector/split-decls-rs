macro_rules! deps {
    () => {
        ToSqlOutput!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_313 {
    () => {
        deps!();
        # [doc = " UTC time => UTC RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DD HH:MM:SS.SSS+00:00\")."] impl ToSql for DateTime < Utc > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F %T%.f%:z") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_313!()