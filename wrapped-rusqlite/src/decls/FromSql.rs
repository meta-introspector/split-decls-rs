macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlResult!();
    };
}

macro_rules! FromSql {
    () => {
        deps!();
        # [doc = " A trait for types that can be created from a SQLite value."] pub trait FromSql : Sized { # [doc = " Converts SQLite value into Rust value."] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > ; }
    };
}

FromSql!()