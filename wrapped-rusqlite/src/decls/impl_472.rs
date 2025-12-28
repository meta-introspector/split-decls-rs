macro_rules! deps {
    () => {
        ToSqlOutput!();
        Null!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_472 {
    () => {
        deps!();
        impl < T : ToSql > ToSql for Option < T > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { match * self { None => Ok (ToSqlOutput :: from (Null)) , Some (ref t) => t . to_sql () , } } }
    };
}

impl_472!();