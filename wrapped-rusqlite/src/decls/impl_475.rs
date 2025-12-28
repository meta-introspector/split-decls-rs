macro_rules! deps {
    () => {
        Result!();
        ToSqlOutput!();
        ToSql!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        # [doc = " Serialize `Url` to text."] impl ToSql for Url { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . as_str ())) } }
    };
}

impl_475!();