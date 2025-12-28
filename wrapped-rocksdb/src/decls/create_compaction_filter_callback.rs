macro_rules! deps {
    () => {
        CompactionFilterFactory!();
        CompactionFilterContext!();
    };
}

macro_rules! create_compaction_filter_callback {
    () => {
        deps!();
        pub unsafe extern "C" fn create_compaction_filter_callback < F > (raw_self : * mut c_void , context : * mut ffi :: rocksdb_compactionfiltercontext_t ,) -> * mut ffi :: rocksdb_compactionfilter_t where F : CompactionFilterFactory , { let self_ = unsafe { & mut * (raw_self as * mut F) } ; let context = unsafe { CompactionFilterContext :: from_raw (context) } ; let filter = Box :: new (self_ . create (context)) ; let filter_ptr = Box :: into_raw (filter) ; unsafe { ffi :: rocksdb_compactionfilter_create (filter_ptr as * mut c_void , Some (compaction_filter :: destructor_callback :: < F :: Filter >) , Some (compaction_filter :: filter_callback :: < F :: Filter >) , Some (compaction_filter :: name_callback :: < F :: Filter >) ,) } }
    };
}

create_compaction_filter_callback!()