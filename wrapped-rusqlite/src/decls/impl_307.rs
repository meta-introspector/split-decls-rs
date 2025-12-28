macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        ToSql!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        # [doc = " ISO 8601 calendar date without timezone => \"YYYY-MM-DD\""] impl ToSql for NaiveDate { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format ("%F") . to_string () ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_307!()