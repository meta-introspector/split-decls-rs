macro_rules! deps {
    () => {
        ToSql!();
        Result!();
        ToSqlOutput!();
        Array!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl ToSql for Array { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: Array (self . clone ())) } }
    };
}

impl_610!();