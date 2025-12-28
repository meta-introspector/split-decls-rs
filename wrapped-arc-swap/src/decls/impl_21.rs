macro_rules! deps {
    () => {
        ArcSwapAny!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : RefCnt , S : Strategy < T > > Drop for ArcSwapAny < T , S > { fn drop (& mut self) { let ptr = * self . ptr . get_mut () ; unsafe { self . strategy . wait_for_readers (ptr , & self . ptr) ; T :: dec (ptr) ; } } }
    };
}

impl_21!()