macro_rules! OrderByIter {
    () => {
        # [doc = " `feature = \"vtab\"`"] pub struct OrderByIter < 'a > { iter : slice :: Iter < 'a , ffi :: sqlite3_index_orderby > , }
    };
}

OrderByIter!();