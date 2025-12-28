macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        Error!();
        ToSqlOutput!();
        Value!();
    };
}

macro_rules! to_sql_self_fallible {
    () => {
        deps!();
        # [cfg (feature = "fallible_uint")] macro_rules ! to_sql_self_fallible (($ t : ty) => (impl ToSql for $ t { # [inline] fn to_sql (& self) -> Result < ToSqlOutput <'_ >> { Ok (ToSqlOutput :: Owned (Value :: Integer (i64 :: try_from (* self) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ?))) } }) ; (non_zero $ t : ty) => (impl ToSql for $ t { # [inline] fn to_sql (& self) -> Result < ToSqlOutput <'_ >> { Ok (ToSqlOutput :: Owned (Value :: Integer (i64 :: try_from (self . get ()) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) ?))) } })) ;
    };
}

to_sql_self_fallible!();