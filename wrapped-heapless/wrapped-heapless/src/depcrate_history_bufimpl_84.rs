// Generated macro for impl_84 (impl)
macro_rules! Depcrate_history_bufimpl_84 {
() => {
// Module: crate::history_buf
// Provides: {"impl_84"}
// Dependencies: {}
impl < T : Copy , S : HistoryBufStorage < T > + ? Sized > HistoryBufInner < T , S > { # [doc = " Get a reference to the `HistoryBuf`, erasing the `N` const-generic."] # [inline] pub fn as_view (& self) -> & HistoryBufView < T > { S :: as_hist_buf_view (self) } # [doc = " Get a mutable reference to the `HistoryBuf`, erasing the `N` const-generic."] # [inline] pub fn as_mut_view (& mut self) -> & mut HistoryBufView < T > { S :: as_hist_buf_mut_view (self) } # [doc = " Clears the buffer, replacing every element with the given value."] pub fn clear_with (& mut self , t : T) { unsafe { self . drop_contents () } ; self . write_at = 0 ; self . filled = true ; for d in self . data . borrow_mut () { * d = MaybeUninit :: new (t) ; } } }
};
}
