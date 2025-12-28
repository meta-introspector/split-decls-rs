macro_rules! deps {
    () => {
        ToSql!();
        ToSqlOutput!();
        Result!();
        ValueRef!();
        Null!();
        Error!();
        Value!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        # [doc = " Serialize JSON `Value` to text:"] # [doc = ""] # [doc = ""] # [doc = " | JSON   | SQLite    |"] # [doc = " |----------|---------|"] # [doc = " | Null     | NULL    |"] # [doc = " | Bool     | 'true' / 'false' |"] # [doc = " | Number   | INT or REAL except u64 |"] # [doc = " | _ | TEXT |"] impl ToSql for Value { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { match self { Self :: Null => Ok (ToSqlOutput :: Borrowed (ValueRef :: Null)) , Self :: Number (n) if n . is_i64 () => Ok (ToSqlOutput :: from (n . as_i64 () . unwrap ())) , Self :: Number (n) if n . is_f64 () => Ok (ToSqlOutput :: from (n . as_f64 () . unwrap ())) , _ => serde_json :: to_string (self) . map (ToSqlOutput :: from) . map_err (| err | Error :: ToSqlConversionFailure (err . into ())) , } } }
    };
}

impl_379!();