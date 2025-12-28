macro_rules! deps {
    () => {
        UnsafeFutureObj!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        unsafe impl < 'a , T , F > UnsafeFutureObj < 'a , T > for & 'a mut F where F : Future < Output = T > + Unpin + 'a , { fn into_raw (self) -> * mut (dyn Future < Output = T > + 'a) { self as * mut dyn Future < Output = T > } unsafe fn drop (_ptr : * mut (dyn Future < Output = T > + 'a)) { } }
    };
}

impl_46!()