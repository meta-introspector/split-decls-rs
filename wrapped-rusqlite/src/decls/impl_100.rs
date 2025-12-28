macro_rules! deps {
    () => {
        ToSql!();
        SqlFnArg!();
        Result!();
        ToSqlOutput!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl ToSql for SqlFnArg { fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: Arg (self . idx)) } }
    };
}

impl_100!()