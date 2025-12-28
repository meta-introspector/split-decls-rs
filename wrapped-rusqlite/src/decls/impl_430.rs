macro_rules! deps {
    () => {
        Result!();
        ToSql!();
        Array!();
        ValueRef!();
        ToSqlOutput!();
        ZeroBlob!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        impl ToSql for ToSqlOutput < '_ > { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (match * self { ToSqlOutput :: Borrowed (v) => ToSqlOutput :: Borrowed (v) , ToSqlOutput :: Owned (ref v) => ToSqlOutput :: Borrowed (ValueRef :: from (v)) , # [cfg (feature = "blob")] ToSqlOutput :: ZeroBlob (i) => ToSqlOutput :: ZeroBlob (i) , # [cfg (feature = "functions")] ToSqlOutput :: Arg (i) => ToSqlOutput :: Arg (i) , # [cfg (feature = "array")] ToSqlOutput :: Array (ref a) => ToSqlOutput :: Array (a . clone ()) , }) } }
    };
}

impl_430!();