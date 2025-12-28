macro_rules! IndexConstraintIter {
    () => {
        # [doc = " `feature = \"vtab\"`"] pub struct IndexConstraintIter < 'a > { iter : slice :: Iter < 'a , ffi :: sqlite3_index_constraint > , }
    };
}

IndexConstraintIter!()