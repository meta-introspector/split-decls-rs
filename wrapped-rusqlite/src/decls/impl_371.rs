macro_rules! deps {
    () => {
        Result!();
        ToSqlOutput!();
        ToSql!();
    };
}

macro_rules! impl_371 {
    () => {
        deps!();
        # [doc = " time => \"HH:MM:SS.SSS\""] impl ToSql for Time { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_371!();