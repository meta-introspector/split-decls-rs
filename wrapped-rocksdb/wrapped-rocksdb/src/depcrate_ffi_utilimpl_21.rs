// Generated macro for impl_21 (impl)
macro_rules! Depcrate_ffi_utilimpl_21 {
() => {
// Module: crate::ffi_util
// Provides: {"impl_21"}
// Dependencies: {}
impl CSlice { # [doc = " Constructing such a slice may be unsafe."] # [doc = ""] # [doc = " # Safety"] # [doc = " The caller must ensure that the pointer and length are valid."] # [doc = " Moreover, `CSlice` takes the ownership of the memory and will free it"] # [doc = " using `rocksdb_free`. The caller must ensure that the memory is"] # [doc = " allocated by `malloc` in RocksDB and will not be freed by any other"] # [doc = " means."] pub (crate) unsafe fn from_raw_parts (data : * const c_char , len : size_t) -> Self { Self { data , len } } }
};
}
