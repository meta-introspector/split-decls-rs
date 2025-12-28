macro_rules! deps {
    () => {
        MultiThreaded!();
        UnboundColumnFamily!();
        ThreadMode!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl ThreadMode for MultiThreaded { fn new_cf_map_internal (cfs : BTreeMap < String , * mut ffi :: rocksdb_column_family_handle_t > ,) -> Self { Self { cfs : RwLock :: new (cfs . into_iter () . map (| (n , c) | (n , Arc :: new (UnboundColumnFamily { inner : c }))) . collect () ,) , } } fn drop_all_cfs_internal (& mut self) { self . cfs . write () . unwrap () . clear () ; } }
    };
}

impl_98!()