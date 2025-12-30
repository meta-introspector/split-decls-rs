// Generated macro for impl_48 (impl)
macro_rules! Depcrate_rawimpl_48 {
() => {
// Module: crate::raw
// Provides: {"impl_48"}
// Dependencies: {}
impl < F , T > Drop for Guard < F , T > where F : Future < Output = T > , { fn drop (& mut self) { let ptr = self . 0 ; let task_layout = self . 1 ; let header = ptr as * const Header ; unsafe { let header = & * header ; let mut state = header . state . load (Ordering :: Acquire) ; loop { if state & CLOSED != 0 { drop_future :: < F > (ptr , task_layout) ; header . state . fetch_and (! RUNNING & ! SCHEDULED , Ordering :: AcqRel) ; let mut awaiter = None ; if state & AWAITER != 0 { awaiter = header . take (None) ; } drop_ref (ptr) ; if let Some (w) = awaiter { abort_on_panic (| | w . wake ()) ; } break ; } match header . state . compare_exchange_weak (state , (state & ! RUNNING & ! SCHEDULED) | CLOSED , Ordering :: AcqRel , Ordering :: Acquire ,) { Ok (state) => { drop_future :: < F > (ptr , task_layout) ; let mut awaiter = None ; if state & AWAITER != 0 { awaiter = header . take (None) ; } drop_ref (ptr) ; if let Some (w) = awaiter { abort_on_panic (| | w . wake ()) ; } break ; } Err (s) => state = s , } } } } }
};
}
