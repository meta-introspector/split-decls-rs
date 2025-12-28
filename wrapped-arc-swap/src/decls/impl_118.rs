macro_rules! deps {
    () => {
        RefCnt!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < T : RefCnt > InnerStrategy < T > for RwLock < () > { type Protected = T ; unsafe fn load (& self , storage : & AtomicPtr < T :: Base >) -> T { let _guard = self . read () . expect ("We don't panic in here") ; let ptr = storage . load (Ordering :: Acquire) ; let ptr = T :: from_ptr (ptr as * const T :: Base) ; T :: inc (& ptr) ; ptr } unsafe fn wait_for_readers (& self , _ : * const T :: Base , _ : & AtomicPtr < T :: Base >) { drop (self . write () . expect ("We don't panic in here")) ; } }
    };
}

impl_118!()