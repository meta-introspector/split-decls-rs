macro_rules! deps {
    () => {
        ValueRef!();
        FromSqlResult!();
        FromSql!();
        FromSqlError!();
    };
}

macro_rules! impl_374 {
    () => {
        deps!();
        # [doc = " \"YYYY-MM-DDTHH:MM:SS.SSS\" => Gregorian datetime."] impl FromSql for DateTime { fn column_result (value : ValueRef < '_ >) -> FromSqlResult < Self > { value . as_str () . and_then (| s | s . parse () . map_err (FromSqlError :: other)) } }
    };
}

impl_374!()