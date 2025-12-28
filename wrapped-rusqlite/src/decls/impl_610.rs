macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        Array!();
        ToSqlOutput!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl ToSql for Array { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: Array (self . clone ())) } }
    };
}

impl_610!()