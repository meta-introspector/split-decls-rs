// Generated macro for impl_31 (impl)
macro_rules! Depcrate_cacheimpl_31 {
() => {
// Module: crate::cache
// Provides: {"impl_31"}
// Dependencies: {}
impl < S , R > AsyncCache < S , R > where S : Stream , { fn poll_next_item (& self , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { let pin = unsafe { Pin :: new_unchecked (& self . stream) } ; let poll = PinMut :: as_mut (& mut pin . borrow_mut ()) . poll_next (cx) ; if poll . is_ready () { let wakers = std :: mem :: take (& mut * self . pending_wakes . borrow_mut ()) ; for waker in wakers { waker . wake () ; } } else { self . pending_wakes . borrow_mut () . push (cx . waker () . clone ()) ; } poll } }
};
}
