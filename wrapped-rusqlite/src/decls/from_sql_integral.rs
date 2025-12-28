macro_rules! deps {
    () => {
        FromSqlResult!();
        ValueRef!();
        FromSql!();
        FromSqlError!();
    };
}

macro_rules! from_sql_integral {
    () => {
        deps!();
        macro_rules ! from_sql_integral (($ t : ident) => (impl FromSql for $ t { # [inline] fn column_result (value : ValueRef <'_ >) -> FromSqlResult < Self > { let i = i64 :: column_result (value) ?; i . try_into () . map_err (| _ | FromSqlError :: OutOfRange (i)) } }) ; (non_zero $ nz : ty , $ z : ty) => (impl FromSql for $ nz { # [inline] fn column_result (value : ValueRef <'_ >) -> FromSqlResult < Self > { let i = <$ z >:: column_result (value) ?; <$ nz >:: new (i) . ok_or (FromSqlError :: OutOfRange (0)) } })) ;
    };
}

from_sql_integral!()