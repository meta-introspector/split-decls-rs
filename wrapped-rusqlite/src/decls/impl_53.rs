macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        ToSqlOutput!();
        ZeroBlob!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl ToSql for ZeroBlob { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let Self (length) = * self ; Ok (ToSqlOutput :: ZeroBlob (length)) } }
    };
}

impl_53!()