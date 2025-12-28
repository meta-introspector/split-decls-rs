macro_rules! deps {
    () => {
        ToSql!();
        ToSqlOutput!();
        Result!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < T : ToSql + ToOwned + ? Sized > ToSql for Cow < '_ , T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
    };
}

impl_432!()