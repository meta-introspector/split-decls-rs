macro_rules! deps {
    () => {
        ToSqlOutput!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl ToSql for str { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self)) } }
    };
}

impl_467!()