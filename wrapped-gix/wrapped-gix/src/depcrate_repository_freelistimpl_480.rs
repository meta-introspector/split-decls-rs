// Generated macro for impl_480 (impl)
macro_rules! Depcrate_repository_freelistimpl_480 {
() => {
// Module: crate::repository::freelist
// Provides: {"impl_480"}
// Dependencies: {}
# [doc = " Internal"] impl crate :: Repository { # [doc = " Note that the returned buffer might still have data in it."] # [inline] pub (crate) fn free_buf (& self) -> Vec < u8 > { self . bufs . as_ref () . and_then (| bufs | bufs . borrow_mut () . pop ()) . unwrap_or_default () } # [doc = " This method is commonly called from the destructor of objects that previously claimed an entry"] # [doc = " in the free-list with [crate::Repository::free_buf]."] # [doc = " They are welcome to take out the data themselves, for instance when the object is detached, to avoid"] # [doc = " it to be reclaimed."] # [inline] pub (crate) fn reuse_buffer (& self , data : & mut Vec < u8 >) { if data . capacity () > 0 { if let Some (bufs) = self . bufs . as_ref () { bufs . borrow_mut () . push (std :: mem :: take (data)) ; } } } }
};
}
