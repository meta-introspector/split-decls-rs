macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlResult!();
        FromSql!();
    };
}

macro_rules! impl_355 {
    () => {
        deps!();
        impl FromSql for std :: rc :: Rc < str > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . map (Into :: into) } }
    };
}

impl_355!()