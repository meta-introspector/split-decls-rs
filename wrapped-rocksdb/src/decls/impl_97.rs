macro_rules! deps {
    () => {
        ThreadMode!();
        SingleThreaded!();
        ColumnFamily!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl ThreadMode for SingleThreaded { fn new_cf_map_internal (cfs : BTreeMap < String , * mut ffi :: rocksdb_column_family_handle_t > ,) -> Self { Self { cfs : cfs . into_iter () . map (| (n , c) | (n , ColumnFamily { inner : c })) . collect () , } } fn drop_all_cfs_internal (& mut self) { self . cfs . clear () ; } }
    };
}

impl_97!()