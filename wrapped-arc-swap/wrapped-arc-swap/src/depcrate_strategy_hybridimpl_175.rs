// Generated macro for impl_175 (impl)
macro_rules! Depcrate_strategy_hybridimpl_175 {
() => {
// Module: crate::strategy::hybrid
// Provides: {"impl_175"}
// Dependencies: {}
impl < T : RefCnt , Cfg : Config > CaS < T > for HybridStrategy < Cfg > { unsafe fn compare_and_swap < C : crate :: as_raw :: AsRaw < T :: Base > > (& self , storage : & AtomicPtr < T :: Base > , current : C , new : T ,) -> Self :: Protected { loop { let old = < Self as InnerStrategy < T > > :: load (self , storage) ; if old . as_ptr () != current . as_raw () { return old ; } let new_raw = T :: as_ptr (& new) ; if storage . compare_exchange_weak (current . as_raw () , new_raw , SeqCst , Relaxed) . is_ok () { T :: into_ptr (new) ; < Self as InnerStrategy < T > > :: wait_for_readers (self , old . as_ptr () , storage) ; T :: dec (old . as_ptr ()) ; return old ; } } } }
};
}
