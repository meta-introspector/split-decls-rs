// Generated macro for impl_2246 (impl)
macro_rules! Depcrate_mysql_connection_bindimpl_2246 {
() => {
// Module: crate::mysql::connection::bind
// Provides: {"impl_2246"}
// Dependencies: {}
impl Clone for BindData { fn clone (& self) -> Self { let (ptr , len , capacity) = if let Some (ptr) = self . bytes { let slice = unsafe { std :: slice :: from_raw_parts (ptr . as_ptr () , self . length . try_into () . expect ("usize is at least 32bit") ,) } ; let mut vec = slice . to_owned () ; let ptr = NonNull :: new (vec . as_mut_ptr ()) ; let len = vec . len () as libc :: c_ulong ; let capacity = vec . capacity () ; mem :: forget (vec) ; (ptr , len , capacity) } else { (None , 0 , 0) } ; Self { tpe : self . tpe , bytes : ptr , length : len , capacity , flags : self . flags , is_null : self . is_null , is_truncated : self . is_truncated , } } }
};
}
