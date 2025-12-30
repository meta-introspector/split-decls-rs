// Generated macro for impl_174 (impl)
macro_rules! Depcrate_sync_listimpl_174 {
() => {
// Module: crate::sync::list
// Provides: {"impl_174"}
// Dependencies: {}
impl Entry { # [doc = " Marks this entry as deleted, deferring the actual deallocation to a later iteration."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The entry should be a member of a linked list, and it should not have been deleted."] # [doc = " It should be safe to call `C::finalize` on the entry after the `guard` is dropped, where `C`"] # [doc = " is the associated helper for the linked list."] pub (crate) unsafe fn delete (& self , guard : & Guard) { self . next . fetch_or (1 , Release , guard) ; } }
};
}
