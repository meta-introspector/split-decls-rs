macro_rules! deps {
    () => {
        Result!();
        Error!();
        ToSqlOutput!();
        ToSql!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        # [doc = " ISO 8601 calendar date without timezone => \"YYYY-MM-DD\""] impl ToSql for Date { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_str = self . format (& DATE_FORMAT) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (date_str)) } }
    };
}

impl_394!();