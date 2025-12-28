macro_rules! deps {
    () => {
        CompactionFilterContext!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl CompactionFilterContext { unsafe fn from_raw (ptr : * mut ffi :: rocksdb_compactionfiltercontext_t) -> Self { let is_full_compaction = unsafe { ffi :: rocksdb_compactionfiltercontext_is_full_compaction (ptr) } != 0 ; let is_manual_compaction = unsafe { ffi :: rocksdb_compactionfiltercontext_is_manual_compaction (ptr) } != 0 ; Self { is_full_compaction , is_manual_compaction , } } }
    };
}

impl_80!()