macro_rules! deps {
    () => {
        ToSql!();
        ToSqlOutput!();
        Result!();
        Error!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        # [doc = " `OffsetDatetime` => RFC3339 format (\"YYYY-MM-DD HH:MM:SS.SSS[+-]HH:MM\")"] impl ToSql for OffsetDateTime { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let time_string = self . format (& OFFSET_DATE_TIME_ENCODING) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ? ; Ok (ToSqlOutput :: from (time_string)) } }
    };
}

impl_392!()