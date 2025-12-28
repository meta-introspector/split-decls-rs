macro_rules! deps {
    () => {
        ToSqlOutput!();
        ToSql!();
        Result!();
    };
}

macro_rules! to_sql_self {
    () => {
        deps!();
        macro_rules ! to_sql_self (($ t : ty) => (impl ToSql for $ t { # [inline] fn to_sql (& self) -> Result < ToSqlOutput <'_ >> { Ok (ToSqlOutput :: from (* self)) } })) ;
    };
}

to_sql_self!()