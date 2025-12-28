macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        ToSqlOutput!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        # [doc = " ISO 8601 combined date and time without timezone =>"] # [doc = " \"YYYY-MM-DD HH:MM:SS.SSS\""] impl ToSql for NaiveDateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F %T%.f") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_311!();