macro_rules! deps {
    () => {
        FromSqlError!();
        Result!();
        FromSql!();
    };
}

macro_rules! FromSqlResult {
    () => {
        deps!();
        # [doc = " Result type for implementors of the [`FromSql`] trait."] pub type FromSqlResult < T > = Result < T , FromSqlError > ;
    };
}

FromSqlResult!();