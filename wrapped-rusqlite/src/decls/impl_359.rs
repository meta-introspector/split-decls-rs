macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlResult!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl FromSql for std :: rc :: Rc < [u8] > { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_blob () . map (std :: rc :: Rc :: < [u8] > :: from) } }
    };
}

impl_359!()