macro_rules! deps {
    () => {
        HybridStrategy!();
        CaS!();
        AsRaw!();
        RefCnt!();
        Config!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < T : RefCnt , Cfg : Config > CaS < T > for HybridStrategy < Cfg > { unsafe fn compare_and_swap < C : crate :: as_raw :: AsRaw < T :: Base > > (& self , storage : & AtomicPtr < T :: Base > , current : C , new : T ,) -> Self :: Protected { loop { let old = < Self as InnerStrategy < T > > :: load (self , storage) ; if old . as_ptr () != current . as_raw () { return old ; } let new_raw = T :: as_ptr (& new) ; if storage . compare_exchange_weak (current . as_raw () , new_raw , SeqCst , Relaxed) . is_ok () { T :: into_ptr (new) ; < Self as InnerStrategy < T > > :: wait_for_readers (self , old . as_ptr () , storage) ; T :: dec (old . as_ptr ()) ; return old ; } } } }
    };
}

impl_114!()