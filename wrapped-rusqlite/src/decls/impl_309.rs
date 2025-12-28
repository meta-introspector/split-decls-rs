macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        ToSql!();
    };
}

macro_rules! impl_309 {
    () => {
        deps!();
        # [doc = " ISO 8601 time without timezone => \"HH:MM:SS.SSS\""] impl ToSql for NaiveTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%T%.f") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_309!()