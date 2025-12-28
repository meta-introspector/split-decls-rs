macro_rules! deps {
    () => {
        ArcJob!();
        Job!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < BODY > Job for ArcJob < BODY > where BODY : Fn () + Send + Sync , { unsafe fn execute (this : * const ()) { unsafe { let this = Arc :: from_raw (this as * mut Self) ; (this . job) () ; } } }
    };
}

impl_45!()