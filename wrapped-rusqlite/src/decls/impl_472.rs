macro_rules! deps {
    () => {
        Result!();
        ToSqlOutput!();
        Null!();
        ToSql!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < T : ToSql > ToSql for Option < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { match * self { None => Ok (ToSqlOutput :: from (Null)) , Some (ref t) => t . to_sql () , } } }
    };
}

impl_472!()