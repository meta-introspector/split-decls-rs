macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        ToSqlOutput!();
    };
}

macro_rules! impl_435 {
    () => {
        deps!();
        impl < T : ToSql + ? Sized > ToSql for std :: sync :: Arc < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
    };
}

impl_435!();