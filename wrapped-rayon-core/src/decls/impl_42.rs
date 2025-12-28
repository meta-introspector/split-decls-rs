macro_rules! deps {
    () => {
        HeapJob!();
        Job!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < BODY > Job for HeapJob < BODY > where BODY : FnOnce () + Send , { unsafe fn execute (this : * const ()) { unsafe { let this = Box :: from_raw (this as * mut Self) ; (this . job) () ; } } }
    };
}

impl_42!()