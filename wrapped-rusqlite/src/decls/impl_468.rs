macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        ToSql!();
    };
}

macro_rules! impl_468 {
    () => {
        deps!();
        impl ToSql for Vec < u8 > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (self . as_slice ())) } }
    };
}

impl_468!()