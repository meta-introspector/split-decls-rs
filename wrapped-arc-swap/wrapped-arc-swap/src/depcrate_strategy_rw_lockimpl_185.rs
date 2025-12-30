// Generated macro for impl_185 (impl)
macro_rules! Depcrate_strategy_rw_lockimpl_185 {
() => {
// Module: crate::strategy::rw_lock
// Provides: {"impl_185"}
// Dependencies: {}
impl < T : RefCnt > CaS < T > for RwLock < () > { unsafe fn compare_and_swap < C : AsRaw < T :: Base > > (& self , storage : & AtomicPtr < T :: Base > , current : C , new : T ,) -> Self :: Protected { let _lock = self . write () ; let cur = current . as_raw () ; let new = T :: into_ptr (new) ; let swapped = storage . compare_exchange (cur , new , Ordering :: AcqRel , Ordering :: Relaxed) ; let old = match swapped { Ok (old) => old , Err (old) => old , } ; let old = T :: from_ptr (old as * const T :: Base) ; if swapped . is_err () { T :: inc (& old) ; drop (T :: from_ptr (new)) ; } drop (current) ; old } }
};
}
