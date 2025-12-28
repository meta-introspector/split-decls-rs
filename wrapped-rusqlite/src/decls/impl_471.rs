macro_rules! deps {
    () => {
        Value!();
        Result!();
        ToSqlOutput!();
        ToSql!();
    };
}

macro_rules! impl_471 {
    () => {
        deps!();
        impl ToSql for Value { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self)) } }
    };
}

impl_471!();