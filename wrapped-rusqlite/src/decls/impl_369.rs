macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        ToSqlOutput!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        # [doc = " Gregorian calendar date => \"YYYY-MM-DD\""] impl ToSql for Date { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let s = self . to_string () ; Ok (ToSqlOutput :: from (s)) } }
    };
}

impl_369!();