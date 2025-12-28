macro_rules! deps {
    () => {
        Result!();
        ToSqlOutput!();
        ToSql!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        # [doc = " Gregorian datetime => \"YYYY-MM-DDTHH:MM:SS.SSS\""] impl ToSql for DateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let s = self . to_string () ; Ok (ToSqlOutput :: from (s)) } }
    };
}

impl_373!()