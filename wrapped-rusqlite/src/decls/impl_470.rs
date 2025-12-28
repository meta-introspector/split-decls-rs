macro_rules! deps {
    () => {
        ToSqlOutput!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_470 {
    () => {
        deps!();
        impl ToSql for [u8] { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self)) } }
    };
}

impl_470!()