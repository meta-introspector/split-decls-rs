macro_rules! deps {
    () => {
        SubType!();
        ToSqlOutput!();
        Result!();
    };
}

macro_rules! SqlFnOutput {
    () => {
        deps!();
        # [doc = " Result of an SQL function"] pub trait SqlFnOutput { # [doc = " Converts Rust value to SQLite value with an optional subtype"] fn to_sql (& self) -> Result < (ToSqlOutput < '_ > , SubType) > ; }
    };
}

SqlFnOutput!();