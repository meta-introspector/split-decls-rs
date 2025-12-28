macro_rules! deps {
    () => {
        FromSql!();
        ValueRef!();
        FromSqlResult!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < T : ? Sized > FromSql for Cow < '_ , T > where T : ToOwned , T :: Owned : FromSql , { # [inline] fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { < T :: Owned > :: column_result (value) . map (Cow :: Owned) } }
    };
}

impl_365!()