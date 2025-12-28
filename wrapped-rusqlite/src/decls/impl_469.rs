macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
        ToSql!();
    };
}

macro_rules! impl_469 {
    () => {
        deps!();
        impl < const N : usize > ToSql for [u8 ; N] { # [inline] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > { Ok (ToSqlOutput :: from (& self [..])) } }
    };
}

impl_469!()