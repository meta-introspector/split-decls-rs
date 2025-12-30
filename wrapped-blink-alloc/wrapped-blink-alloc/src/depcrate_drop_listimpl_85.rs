// Generated macro for impl_85 (impl)
macro_rules! Depcrate_drop_listimpl_85 {
() => {
// Module: crate::drop_list
// Provides: {"impl_85"}
// Dependencies: {}
impl DropList { pub const fn new () -> Self { DropList { root : Cell :: new (None) , } } # [doc = " Adds new drop item for given typed pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `item` reference must be valid until next call to [`DropList::reset`]."] # [allow (clippy :: mut_from_ref)] pub unsafe fn add < 'a , 'b : 'a , T : ? Sized > (& 'a self , item : & 'b mut DropItem < T >) -> & 'a mut T { item . drops . next = self . root . take () ; let item = NonNull :: from (item) ; self . root . set (Some (item . cast ())) ; & mut * addr_of_mut ! ((* item . as_ptr ()) . value) } # [doc = " Drops all items in the list."] pub fn reset (& mut self) { let mut next = self . root . take () ; while let Some (item_ptr) = next { unsafe { next = Drops :: drop (item_ptr) ; } } } }
};
}
