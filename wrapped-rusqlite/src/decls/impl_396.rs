macro_rules! deps {
    () => {
        Error!();
        ToSql!();
        Result!();
        ToSqlOutput!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        # [doc = " ISO 8601 time without timezone => \"HH:MM:SS.SSS\""] impl ToSql for Time { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let time_str = self . format (& TIME_ENCODING) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (time_str)) } }
    };
}

impl_396!()