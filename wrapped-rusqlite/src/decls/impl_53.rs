macro_rules! deps {
    () => {
        Result!();
        ToSqlOutput!();
        ToSql!();
        ZeroBlob!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl ToSql for ZeroBlob { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { let Self (length) = * self ; Ok (ToSqlOutput :: ZeroBlob (length)) } }
    };
}

impl_53!();