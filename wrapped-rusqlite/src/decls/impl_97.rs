macro_rules! deps {
    () => {
        SqlFnOutput!();
        ToSqlOutput!();
        SubType!();
        ToSql!();
        Result!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < T : ToSql > SqlFnOutput for T { # [inline] fn to_sql (& self) -> Result < (ToSqlOutput < '_ > , SubType) > { ToSql :: to_sql (self) . map (| o | (o , None)) } }
    };
}

impl_97!()