macro_rules! OrderBy {
    () => {
        # [doc = " A column of the ORDER BY clause."] pub struct OrderBy < 'a > (& 'a ffi :: sqlite3_index_orderby) ;
    };
}

OrderBy!();