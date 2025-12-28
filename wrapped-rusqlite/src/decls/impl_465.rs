macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        ToSqlOutput!();
    };
}

macro_rules! impl_465 {
    () => {
        deps!();
        impl < T : ? Sized > ToSql for & '_ T where T : ToSql , { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { (* self) . to_sql () } }
    };
}

impl_465!()