macro_rules! IndexConstraint {
    () => {
        # [doc = " WHERE clause constraint."] pub struct IndexConstraint < 'a > (& 'a ffi :: sqlite3_index_constraint) ;
    };
}

IndexConstraint!()