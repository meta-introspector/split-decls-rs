macro_rules! deps {
    () => {
        ToSqlOutput!();
        Result!();
    };
}

macro_rules! ToSql {
    () => {
        deps!();
        # [doc = " A trait for types that can be converted into SQLite values. Returns"] # [doc = " [`crate::Error::ToSqlConversionFailure`] if the conversion fails."] pub trait ToSql { # [doc = " Converts Rust value to SQLite value"] fn to_sql (& self) -> Result < ToSqlOutput < '_ > > ; }
    };
}

ToSql!()