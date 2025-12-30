// Generated macro for impl_481 (impl)
macro_rules! Depcrate_repository_freelistimpl_481 {
() => {
// Module: crate::repository::freelist
// Provides: {"impl_481"}
// Dependencies: {}
# [doc = " Freelist configuration"] # [doc = ""] # [doc = " The free-list is an internal and 'transparent' mechanism for obtaining and re-using memory buffers when"] # [doc = " reading objects. That way, trashing is avoided as buffers are re-used and re-written."] # [doc = ""] # [doc = " However, there are circumstances when releasing memory early is preferred, for instance on the server side."] # [doc = ""] # [doc = " Also note that the free-list isn't cloned, so each clone of this instance starts with an empty one."] impl crate :: Repository { # [doc = " Return an empty buffer which is tied to this repository instance, and reuse its memory allocation by"] # [doc = " keeping it around even after it drops."] pub fn empty_reusable_buffer (& self) -> Buffer < '_ > { let mut inner = self . free_buf () ; inner . clear () ; Buffer { inner , repo : self } } # [doc = " Set the currently used freelist to `list`. If `None`, it will be disabled entirely."] # [doc = ""] # [doc = " Return the currently previously allocated free-list, a list of reusable buffers typically used when reading objects."] # [doc = " May be `None` if there was no free-list."] pub fn set_freelist (& mut self , list : Option < Vec < Vec < u8 > > >) -> Option < Vec < Vec < u8 > > > { let previous = self . bufs . take () ; self . bufs = list . map (RefCell :: new) ; previous . map (RefCell :: into_inner) } # [doc = " A builder method to disable the free-list on a newly created instance."] pub fn without_freelist (mut self) -> Self { self . bufs . take () ; self } }
};
}
