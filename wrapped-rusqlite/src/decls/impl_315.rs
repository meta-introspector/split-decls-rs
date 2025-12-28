macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        ToSql!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        # [doc = " Date and time with time zone => RFC3339 timestamp"] # [doc = " (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\")."] impl ToSql for DateTime < FixedOffset > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F %T%.f%:z") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_315!()