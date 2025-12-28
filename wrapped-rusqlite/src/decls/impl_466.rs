macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        ToSqlOutput!();
    };
}

macro_rules! impl_466 {
    () => {
        deps!();
        impl ToSql for String { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . as_str ())) } }
    };
}

impl_466!();