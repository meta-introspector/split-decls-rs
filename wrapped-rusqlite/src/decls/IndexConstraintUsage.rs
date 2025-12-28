macro_rules! IndexConstraintUsage {
    () => {
        # [doc = " Information about what parameters to pass to"] # [doc = " [`VTabCursor::filter`]."] pub struct IndexConstraintUsage < 'a > (& 'a mut ffi :: sqlite3_index_constraint_usage) ;
    };
}

IndexConstraintUsage!()