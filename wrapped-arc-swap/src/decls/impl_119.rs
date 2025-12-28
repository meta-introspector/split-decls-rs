macro_rules! deps {
    () => {
        CaS!();
        RefCnt!();
        AsRaw!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < T : RefCnt > CaS < T > for RwLock < () > { unsafe fn compare_and_swap < C : AsRaw < T :: Base > > (& self , storage : & AtomicPtr < T :: Base > , current : C , new : T ,) -> Self :: Protected { let _lock = self . write () ; let cur = current . as_raw () ; let new = T :: into_ptr (new) ; let swapped = storage . compare_exchange (cur , new , Ordering :: AcqRel , Ordering :: Relaxed) ; let old = match swapped { Ok (old) => old , Err (old) => old , } ; let old = T :: from_ptr (old as * const T :: Base) ; if swapped . is_err () { T :: inc (& old) ; drop (T :: from_ptr (new)) ; } drop (current) ; old } }
    };
}

impl_119!()