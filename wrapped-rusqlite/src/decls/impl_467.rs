macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        ToSqlOutput!();
    };
}

macro_rules! impl_467 {
    () => {
        deps!();
        impl ToSql for str { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self)) } }
    };
}

impl_467!();