macro_rules! deps {
    () => {
        SubType!();
        ToSqlOutput!();
        SqlFnOutput!();
        Result!();
        ToSql!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < T : ToSql > SqlFnOutput for (T , SubType) { fn to_sql (& self) -> Result < (ToSqlOutput < '_ > , SubType) > { ToSql :: to_sql (& self . 0) . map (| o | (o , self . 1)) } }
    };
}

impl_98!()