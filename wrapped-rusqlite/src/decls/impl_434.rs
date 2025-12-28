macro_rules! deps {
    () => {
        Result!();
        ToSqlOutput!();
        ToSql!();
    };
}

macro_rules! impl_434 {
    () => {
        deps!();
        impl < T : ToSql + ? Sized > ToSql for std :: rc :: Rc < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { self . as_ref () . to_sql () } }
    };
}

impl_434!()