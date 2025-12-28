macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        Error!();
        ToSql!();
    };
}

macro_rules! impl_398 {
    () => {
        deps!();
        # [doc = " ISO 8601 combined date and time without timezone => \"YYYY-MM-DD HH:MM:SS.SSS\""] impl ToSql for PrimitiveDateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let date_time_str = self . format (& PRIMITIVE_DATE_TIME_ENCODING) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (date_time_str)) } }
    };
}

impl_398!()