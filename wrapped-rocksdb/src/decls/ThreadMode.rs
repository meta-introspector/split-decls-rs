macro_rules! deps {
    () => {
        SingleThreaded!();
        DBWithThreadMode!();
        MultiThreaded!();
    };
}

macro_rules! ThreadMode {
    () => {
        deps!();
        # [doc = " Marker trait to specify single or multi threaded column family alternations for"] # [doc = " [`DBWithThreadMode<T>`]"] # [doc = ""] # [doc = " This arrangement makes differences in self mutability and return type in"] # [doc = " some of `DBWithThreadMode` methods."] # [doc = ""] # [doc = " While being a marker trait to be generic over `DBWithThreadMode`, this trait"] # [doc = " also has a minimum set of not-encapsulated internal methods between"] # [doc = " [`SingleThreaded`] and [`MultiThreaded`].  These methods aren't expected to be"] # [doc = " called and defined externally."] pub trait ThreadMode { # [doc = " Internal implementation for storing column family handles"] fn new_cf_map_internal (cf_map : BTreeMap < String , * mut ffi :: rocksdb_column_family_handle_t > ,) -> Self ; # [doc = " Internal implementation for dropping column family handles"] fn drop_all_cfs_internal (& mut self) ; }
    };
}

ThreadMode!();